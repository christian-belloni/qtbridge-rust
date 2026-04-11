// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::path::{Path, PathBuf};
use std::fs;

use qtbridge_build_common::file_system_utils::{absolute_path, create_dirs, find_all_files, write_to_file};
use qtbridge_build_common::generate_types::{CodeFile, FileTree, GenerateFiles, RustFileInfo, get_header};
use qtbridge_gen_common::format_code::{format_rust_code, try_format_cpp_code};
use qtbridge_gen_common::naming;
use qtbridge_gen_common::path_utils::relative_input_file_path_to_path_qualified;
use qt_type_gen_lib::bridge::BridgeTypesGenerator;
use qt_type_gen_lib::generate_type_info::generate_qt_types_getters_code;

const INPUT_ROOT: &str = "src/input";
const DEST_CRATE_ROOT: &str = "../../crates/qtbridge-type-lib";

struct TypeGenerator {
    gen_impl: BridgeTypesGenerator,
}

impl TypeGenerator {
    fn new(input_root: PathBuf) -> Self {
        Self {
            gen_impl: BridgeTypesGenerator::new(input_root),
        }
    }

    fn check_unresolved_dependencies(&self) -> Result<(), String> {
        let unresolved = self.gen_impl.get_pending_dependencies();
        if !unresolved.is_empty() {
            return Err(format!("Unresolved types during generation:\n {}", unresolved.join(", ")))
        }
        Ok(())
    }

    /// Store file with code initializing Qt types list.
    fn store_qt_type_init_code(file_path: &Path) -> Result<(), String> {
        let header = get_header("", Self::package_name());
        let code_tokens = generate_qt_types_getters_code()?;
        let mut code_str = format_rust_code(&code_tokens)?;
        code_str.insert_str(0, &header);

        write_to_file(file_path, &code_str)
    }
}

impl GenerateFiles for TypeGenerator {

    fn package_name() -> &'static str {
        "qt_type_gen"
    }

    fn process_file(&mut self, input_path: &Path) -> Result<FileTree, String> {
        let generated_submodules = self.gen_impl.process_input_file(input_path)?;

        let mut out_tree = FileTree::new();

        for submod in generated_submodules {

            let mod_path_rust = PathBuf::from(relative_input_file_path_to_path_qualified(&submod.input_file_path)?
                .join("/"));
            let mod_path_cpp = mod_path_rust.join("cpp");

            let rust_filepath = mod_path_rust.join(naming::rust::filename::type_gen_file(&submod.name));
            let rust_code = format_rust_code(&submod.code.rust)?;
            let mut local_reexports = Vec::new();
            let mut global_reexports = Vec::new();
            submod.reexport.all().iter()
                .for_each(|ident| {
                    let ident_str = ident.to_string();
                    local_reexports.push(format!("pub use {}::{};", &submod.name, ident_str));
                    global_reexports.push(ident_str);
                });


            out_tree.insert(rust_filepath,
                CodeFile::new_rust(rust_code, Some(submod.input_file_path.clone()), RustFileInfo {
                    has_cxx_bridge: submod.is_cxx_present,
                    is_pub_mod: true,
                    local_reexports,
                    global_mod_idents: global_reexports,
                }));

            if !submod.code.cpp_header.is_empty() {
                let header_filepath = mod_path_cpp.join(naming::cpp::filename::type_gen_header(&submod.name));
                let header_code = try_format_cpp_code(&submod.code.cpp_header)?;
                out_tree.insert(header_filepath, CodeFile::new_header(header_code, Some(submod.input_file_path.clone())));
            }

            if !submod.code.cpp_src.is_empty() {
                let cpp_filepath = mod_path_cpp.join(naming::cpp::filename::type_gen_cpp(&submod.name));
                let cpp_code = try_format_cpp_code(&submod.code.cpp_src)?;
                out_tree.insert(cpp_filepath, CodeFile::new_cpp(cpp_code, Some(submod.input_file_path.clone())));
            }
        }

        Ok(out_tree)
    }
}

fn main() {
    let input_root = PathBuf::from(INPUT_ROOT);

    // First generate files in OUT_DIR of this project.
    // Later if everything is Ok, move generated files to the destination.
    let out_dir_var = std::env::var("OUT_DIR")
        .map_err(|err| format!("Failed to get 'OUT_DIR' environment variable.\nError: {err}"))
        .unwrap();
    let staging_root = absolute_path(&PathBuf::from(out_dir_var))
        .unwrap()
        .join("type_gen");

    let dst_crate_root = PathBuf::from(DEST_CRATE_ROOT);

    // Generate code for Qt types
    let mut generator = TypeGenerator::new(input_root.clone());
    let generator_output = generator.generate_files(&input_root, &staging_root, true)
        .unwrap();

    generator.check_unresolved_dependencies()
        .unwrap();

    let mut dest = dst_crate_root.join("generated_files_bridge.rs");
    fs::write(&dest, &generator_output.generated_files_bridge_code).unwrap();
    dest = dst_crate_root.join("generated_files_cpp.rs");
    fs::write(&dest, &generator_output.generated_files_cpp_code).unwrap();

    generator.place_files(&dst_crate_root, &generator_output)
        .unwrap();

    // Mark build as dirty if any of input files was changed
    find_all_files(&input_root, true)
        .unwrap()
        .iter()
        .for_each(|path| println!("cargo::rerun-if-changed={}", path.display()));

    let qt_type_info_dir = PathBuf::from(DEST_CRATE_ROOT);
    create_dirs(&qt_type_info_dir)
        .unwrap();
    TypeGenerator::store_qt_type_init_code(&qt_type_info_dir.join("src/qt_types.rs"))
        .unwrap();
}
