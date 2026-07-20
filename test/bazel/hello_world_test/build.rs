fn main() {
    build_print::info!("{}", std::env::current_dir().unwrap().display());

    gen_rcc("qml");
    gen_rcc("CustomColor");
    println!("cargo:rerun-if-changed=qml/Main.qml");
}

fn gen_rcc(name: &str) {
    build_print::info!("gen {name}");
    
    let dest = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap())
            .join(format!("{name}_res.rcc"));

    let stdout = std::process::Command::new("bazel")
        .args(&[
            "run", 
            &format!("//hello_world_test/{name}:export_{name}"), 
            &dest.display().to_string()
        ])
        .output()
        .unwrap();
    build_print::println!("{stdout:?}");
    build_print::println!("");
}
