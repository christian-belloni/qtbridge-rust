// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use proc_macro2::{Span, TokenStream};

use qt_gen_common_no_types::path_utils::file_path_to_module_name;
use qt_gen_common_no_types::type_to_string::path_to_string_fallback;

use quote::ToTokens;

use build_common::file_system_utils::get_relative_path;
use qt_gen_common_no_types::type_registry::QtType;
use qt_gen_common_no_types::type_registry::type_traits::{FindType, MetaTypeId};

use crate::file::{File, Item};
use crate::module::Module;
use crate::reexport::Reexport;

use crate::submod_gen::common::file_path_to_qualified_path_before_name;
use crate::submod_gen::{SubmoduleGenerator, GenericSubmoduleGenerator, NonGenericSubmoduleGenerator, MonomorphedSubmoduleGenerator};


struct PendingSubmoduleGenerator {
    src_file: Rc<crate::file::File>,
    generator: Box<dyn SubmoduleGenerator>,
}

/// Code generated for whole submodule
pub struct SubmoduleGeneratedCode {
    pub rust: TokenStream,
    pub cpp_header: String,
    pub cpp_src: String,
}

/// Type that describes generated submodule (e.g. qhash_qstring_qvariant)
pub struct TypeGeneratedSubmodule {
    pub name: String,
    pub input_file_path: String,
    pub code: SubmoduleGeneratedCode,
    pub reexport: Reexport,
    pub is_cxx_present: bool,
}

pub struct BridgeTypesGenerator {
    input_root: PathBuf,
    pending_submodules: Vec<PendingSubmoduleGenerator>,
}

fn syn_err_to_string(err: syn::Error) -> String {
    format!("Error: '{err}' in code '{}'", err.span().source_text().unwrap_or_default())
}

impl BridgeTypesGenerator {
    pub fn new(input_root: PathBuf) -> Self {
        Self {
            input_root,
            pending_submodules: Vec::new(),
        }
    }

    // Process input file specified by std::Path
    // Resulting vector consist of
    // 0 elements. There are unresolved dependencies for bridge function of given module. Processing needs to be postponed.
    // 1 element. No unresolved dependencies. Input processed.
    // > 1 element. Input processed. This module was the last unresolved dependency for some other module(s) that are also processed.
    pub fn process_input_file(&mut self, input_path: &Path) -> Result<Vec<TypeGeneratedSubmodule>, String> {

        let rel_input_path = get_relative_path(input_path, &self.input_root)?;
        let module_name = file_path_to_module_name(input_path)?;

        let file = self.parse_file(input_path)
            .map_err(|err| format!("Error while trying to process input file '{}':\n {err}", input_path.display()))?;

        let mut qt_bridge_mod_item = None;
        {
            let mut reexport = Reexport::new();
            for item in file.items() {
                match item {
                    Item::QtBridge(module) => qt_bridge_mod_item = Some(module), // Reexport is handled separately
                    Item::CxxBridge(_) => {}, // Nothing to reexport
                    Item::Other(item) => reexport.collect_from_item(item)
                        .map_err(syn_err_to_string)?,
                };
            }
            // Register simple types found in input file so far
            Self::register_non_bridge_types(&reexport, &rel_input_path)?;
        }

        if let Some(qt_bridge_mod_item) = qt_bridge_mod_item {
            // There is #[qt_gen::bridge] module in the file
            let module_ident = qt_bridge_mod_item.ident();
            if *module_ident != module_name {
                return Err(format!("Name of bridge module must match the name of input file ({module_ident} vs {module_name})", ))
            }

            let submodules_generators = Self::create_generators_for_module(Rc::new(qt_bridge_mod_item.clone()), &rel_input_path.to_string_lossy().as_ref())
                .map_err(|err| format!("Error trying to create generators for module {module_name}:\n{err}"))?;

            let file = Rc::new(file);
            let mut result_vec = Vec::new();

            for mut submod_gen in submodules_generators {
                submod_gen.register_type()
                    .map_err(|err| format!("Failed to register type for submodule '{}': {err}", submod_gen.submod_name()))?;

                // Export non bridge types now to reduce number of unresolved dependencies later
                let non_bridge_reexport = submod_gen.get_non_bridge_reexport()
                    .map_err(syn_err_to_string)?;
                Self::register_non_bridge_types(&non_bridge_reexport, &rel_input_path)?;

                let dependencies = submod_gen.get_unresolved_type_dependencies();
                if dependencies.is_empty() {
                    submod_gen.substitute_monomorphed_types_if_needed()
                        .map_err(syn_err_to_string)?;

                    //  Run generation immediately
                    result_vec.extend(self.run_generator(file.clone(), submod_gen)
                        .map_err(|err| format!("Error while generating code:\n{err}"))?);
                }
                else {
                    // Delay running of the generator until dependencies are resolved
                    self.delay_generation(file.clone(), submod_gen);
                }
            }

            Ok(result_vec)
        }
        else {
            // No #[qt_gen::bridge] module in the file
            let mut items_joined = TokenStream::new();
            let mut reexport = Reexport::new();
            for item in file.items() {
                match item {
                    Item::QtBridge(_) => unreachable!(),
                    Item::CxxBridge(item_mod) => item_mod.to_tokens(&mut items_joined),
                    Item::Other(item) => {
                        reexport.collect_from_item(item)
                            .map_err(syn_err_to_string)?;
                        item.to_tokens(&mut items_joined)
                    }
                }
            }

            let is_cxx_present = file.items().iter()
                .any(|item| item.is_cxx_bridge());

            let generated_submodule = TypeGeneratedSubmodule {
                name: module_name.clone(),
                input_file_path: input_path
                    .to_string_lossy()
                    .to_string(),
                code: SubmoduleGeneratedCode {
                    rust: items_joined,
                    cpp_header: String::new(),
                    cpp_src: String::new()
                },
                reexport,
                is_cxx_present,
            };
            //self.register_generated_types(&generated_submodule);

            let mut result = vec![generated_submodule];
            result.extend(self.run_pending_generators()
                .map_err(|err| format!("Error while running pending generators:\nError: {err}"))?);
            Ok(result)
        }

    }

    /// Instantiate submodule generators for every struct generic instantiation if struct is generic
    /// otherwise return single submodule generator.
    /// Register types before generation
    fn create_generators_for_module(src_module: Rc<Module>, input_file_path: &str) -> syn::Result<Vec<Box<dyn SubmoduleGenerator>>> {
        let mut result = Vec::<Box<dyn SubmoduleGenerator>>::new();

        if let Some(struct_) = src_module.structure() {
            let generics = struct_.generics();
            if !generics.is_empty() {
                // Create generator for the generic submodule
                result.push(Box::new(GenericSubmoduleGenerator::new(src_module.clone(), input_file_path)?));

                let inst_decl = struct_.instantiations_declaration()
                    .ok_or_else(|| syn::Error::new(struct_.ident().span(), "Instantiations are not defined"))?;

                // Create generators for required instantiations of the generic struct
                for inst in inst_decl.list() {
                    result.push(Box::new(MonomorphedSubmoduleGenerator::new(src_module.clone(), input_file_path, inst)?));
                }
            }
            else {
                // Create single non generic generator
                result.push(Box::new(NonGenericSubmoduleGenerator::new(src_module.clone(), input_file_path)?));
            }
            return Ok(result)
        }

        // Create single non generic generator for code without structure defined
        result.push(Box::new(NonGenericSubmoduleGenerator::new(src_module, input_file_path)?));
        Ok(result)
    }

    /// Register types that do not require include in CXX block
    fn register_non_bridge_types(reexport: &Reexport, input_path: &Path) -> Result<(), String> {
        let path_in_gen = file_path_to_qualified_path_before_name(input_path)?;

        for ident in reexport.types() {
            let type_name = ident.to_string();
            if QtType::find_by_name(&type_name).is_some() {
                continue
            }

            QtType::add_concrete(type_name, path_in_gen.clone(), MetaTypeId::None, "".into());
        }

        Ok(())
    }

    pub fn get_pending_dependencies(&self) -> Vec<String> {
        let mut result = BTreeSet::new();
        self.pending_submodules.iter()
            .for_each(|submod| {
                result.extend(submod.generator
                        .get_unresolved_type_dependencies().iter()
                        .map(path_to_string_fallback))
                });
        Vec::from_iter(result)
    }

    fn parse_file(&mut self, path: &Path) -> syn::Result<File> {
        let file_content = fs::read_to_string(path)
            .map_err(|err| syn::Error::new(Span::call_site(), format!("Failed to read file content:\n{err}")))?;

        let file: File = syn::parse_str(&file_content)
            .map_err(|err| {
                let mut msg = format!("Failed to parse file. Error:\n{err}\n");
                if let Some(src_span) = err.span().source_text() {
                    msg += &format!("Error code span:\n{}", src_span);
                }
                syn::Error::new(err.span(), msg)
            })?;

        Ok(file)
    }

    fn run_generator(&mut self, src_file: Rc<File>, mut gen_: Box<dyn SubmoduleGenerator>) -> syn::Result<Vec<TypeGeneratedSubmodule>> {

        gen_.check_unclassified_type_tokens()?;

        // Run generator. Construct output structure
        let rust_code = gen_.generate_rust()?;

        // Merge generated qt_gen::bridge code the rest of source file
        // Check if cxx::bridge module is present in source file
        // Get reexports
        let mut is_cxx_present = gen_.is_cxx_present();
        let mut reexport = Reexport::new();
        let mut file_tokens = TokenStream::new();
        for file_item in src_file.items() {
            match file_item {
                Item::QtBridge(_mod) => {
                    reexport.collect_from_token_stream(rust_code.clone())?;
                    rust_code.to_tokens(&mut file_tokens);
                },
                Item::CxxBridge(item_mod) => {
                    is_cxx_present = true;
                    // nothing to reexport from cxx::bridge mod
                    item_mod.to_tokens(&mut file_tokens);
                },
                Item::Other(other_item) => {
                    reexport.collect_from_item(other_item)?;
                    other_item.to_tokens(&mut file_tokens);
                },
            };
        }

        let (header, cpp) = gen_.generate_cpp()?;

        let code = SubmoduleGeneratedCode {
            rust: file_tokens,
            cpp_header: header,
            cpp_src: cpp
        };

        let name = gen_.submod_name();
        let input_file_path = self.input_root.join(gen_.input_file_path())
            .to_string_lossy()
            .to_string();

        let generated_submodule = TypeGeneratedSubmodule {
            name,
            input_file_path,
            code,
            reexport,
            is_cxx_present
        };
        //self.register_generated_types(&generated_submodule);

        let mut result = vec![generated_submodule];
        result.extend(self.run_pending_generators()?);
        Ok(result)
    }

    fn delay_generation(&mut self, src_file: Rc<File>, generator: Box<dyn SubmoduleGenerator>) {
        self.pending_submodules.push(PendingSubmoduleGenerator {
            src_file,
            generator
        });
    }

    fn run_pending_generators(&mut self) -> syn::Result<Vec<TypeGeneratedSubmodule>> {
        let mut result = Vec::new();

        loop {
            for submod in self.pending_submodules.iter_mut() {
                submod.generator.check_unclassified_type_tokens()?;
            }
            let submod_pos = self.pending_submodules.iter()
                .position(|submod| submod.generator.get_unresolved_type_dependencies().is_empty());
            let Some(submod_pos) = submod_pos else {
                break
            };
            let mut submod = self.pending_submodules.swap_remove(submod_pos);
            submod.generator.substitute_monomorphed_types_if_needed()?;
            result.extend(self.run_generator(submod.src_file, submod.generator)?);
        }

        Ok(result)
    }
}

