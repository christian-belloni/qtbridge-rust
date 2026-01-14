// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::collections::BTreeMap;
use std::{env, fs};
use std::path::{Path, PathBuf};

use build_common::file_system_utils::{absolute_path, find_all_files};
use build_common::generate_types::{CodeFile, FileTree, GenerateFiles, RustFileInfo};

use qt_gen_common::format_code::{format_rust_code, try_format_cpp_code};
use qt_gen_common::naming;

use qt_iface_gen_lib::InterfaceDesc;
use qt_iface_gen_lib::proxy_gen::{CppProxyGenerator, CppProxyBridgeGenerator, IfaceTraitGenerator, RustProxyGenerator, RustProxyBridgeGenrator};

const INPUT_ROOT: &'static str = "../qt_iface_gen_lib/src/input";
const DEST_CRATE_ROOT: &'static str = "../../../qt_ifaces";

fn map_syn_err(err: syn::Error) -> String {
    let mut msg = format!("syn::Error occurred:\n{err}");
    if let Some(source) = err.span().source_text() {
        msg.push_str(&format!("\n{source}"));
    }
    msg
}

struct IfacesGenerator {
}

impl IfacesGenerator {
}

impl GenerateFiles for IfacesGenerator{
    fn package_name() -> &'static str {
        "qt_iface_gen"
    }

    fn process_file(&mut self, input_path: &Path) -> Result<FileTree, String> {

        let input_file_str = absolute_path(input_path)?
            .to_string_lossy()
            .to_string();

        // Load interface description
        let iface_name = input_path
            .file_name()
            .ok_or_else(|| format!("File without stem: {}", input_path.display()))?
            .to_string_lossy()
            .to_string();
        let iface = InterfaceDesc::new_from_name_str(&iface_name)
            .map_err(|err| format!("Failed to load interface '{}'.\nError: {err}", iface_name))?;

        let mut result_tree = BTreeMap::new();

        let output_dir_rust = PathBuf::from(naming::rust::module::from_struct_name(&iface_name).to_string());


        // Generate trait defining interface
        {
            let iface_trait_module = naming::rust::module::interface_trait();
            let iface_trait = naming::rust::traits::iface_trait(&iface_name);

            let file_name = naming::rust::filename::vtable();
            let file_path = output_dir_rust.join(file_name);

            let vtable = IfaceTraitGenerator::new(&iface);
            let tokens = vtable.generate()
                .map_err(map_syn_err)?;
            let code = format_rust_code(&tokens)?;

            result_tree.insert(file_path,
                CodeFile::new_rust(code, Some(input_file_str.clone()), RustFileInfo {
                    has_cxx_bridge: false,
                    is_pub_mod: false,
                    local_reexports: vec![format!("pub use {iface_trait_module}::{iface_trait};")],
                    global_mod_idents: vec![],
                }));
        }

        // Generate C++ proxy
        {
            let output_dir_cpp = output_dir_rust.join("cpp");

            let generator = CppProxyGenerator::new(&iface);
            let (mut header_code, mut cpp_code) = generator.generate()
                .map_err(map_syn_err)?;

            header_code = try_format_cpp_code(&header_code)?;
            cpp_code = try_format_cpp_code(&cpp_code)?;

            let header_name = naming::cpp::filename::proxy_header(&iface_name);
            let header_path = output_dir_cpp.join(header_name);
            result_tree.insert(header_path,
                CodeFile::new_header(header_code, Some(input_file_str.clone())));

            let cpp_name = naming::cpp::filename::proxy_cpp(&iface_name);
            let cpp_path = output_dir_cpp.join(cpp_name);
            result_tree.insert(cpp_path,
                CodeFile::new_cpp(cpp_code, Some(input_file_str.clone())));
        }


        // Generate bridge for C++ proxy
        {
            let file_name = naming::rust::filename::proxy_cpp_bridge();
            let file_path = output_dir_rust.join(file_name);

            let module = naming::rust::module::proxy_cpp_bridge();
            let proxy_cpp_struct = naming::cpp::class::proxy_cpp(&iface_name);

            let generator = CppProxyBridgeGenerator::new(&iface);
            let tokens = generator.generate()
                .map_err(map_syn_err)?;
            let code = format_rust_code(&tokens)?;

            result_tree.insert(file_path,
                CodeFile::new_rust(code, Some(input_file_str.clone()), RustFileInfo {
                    has_cxx_bridge: true,
                    is_pub_mod: false,
                    local_reexports: vec![format!("pub use {module}::ffi::{proxy_cpp_struct};")],
                    global_mod_idents: vec![],
                }));
        }

        // Generate Rust proxy
        {
            let file_name = naming::rust::filename::proxy_rust();
            let file_path = output_dir_rust.join(file_name);

            let module = naming::rust::module::proxy_rust();
            let proxy_rust_struct = naming::rust::structure::proxy_rust(&iface_name);

            let generator = RustProxyGenerator::new(&iface);
            let tokens = generator.generate()
                .map_err(map_syn_err)?;
            let code = format_rust_code(&tokens)?;

            result_tree.insert(file_path,
                CodeFile::new_rust(code, Some(input_file_str.clone()), RustFileInfo {
                    has_cxx_bridge: false,
                    is_pub_mod: false,
                    local_reexports: vec![format!("pub use {module}::{proxy_rust_struct};")],
                    global_mod_idents: vec![],
                }));
        }

        // Generate bridge for Rust proxy
        {
            let file_name = naming::rust::filename::proxy_rust_bridge();
            let file_path = output_dir_rust.join(file_name);

            let generator = RustProxyBridgeGenrator::new(&iface);
            let tokens = generator.generate()
                .map_err(map_syn_err)?;
            let code = format_rust_code(&tokens)?;

            result_tree.insert(file_path,
                CodeFile::new_rust(code, Some(input_file_str.clone()), RustFileInfo {
                    has_cxx_bridge: true,
                    is_pub_mod: false,
                    local_reexports: vec![],
                    global_mod_idents: vec![],
                }));
        }

        Ok(result_tree)
    }
}

fn main() {
    let input_root = PathBuf::from(INPUT_ROOT);
    let dst_crate_root = PathBuf::from(DEST_CRATE_ROOT);

    // Generate code for Qt interfaces
    let mut generator = IfacesGenerator {};
    let generator_output = generator.generate_files(&input_root, true)
        .unwrap();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut dest = out_dir.join("generated_files_bridge.rs");
    fs::write(&dest, &generator_output.generated_files_bridge_code).unwrap();
    dest = out_dir.join("generated_files_cpp.rs");
    fs::write(&dest, &generator_output.generated_files_cpp_code).unwrap();

    generator.place_files(&dst_crate_root, &generator_output)
        .unwrap();

    // Mark build as dirty if any input file was changed
    find_all_files(&input_root, true)
        .unwrap()
        .iter()
        .for_each(|path| println!("cargo::rerun-if-changed={}", path.display()));
}
