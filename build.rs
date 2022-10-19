use cmake::Config;
use std::{env, path::PathBuf};

fn main() {
    let dst = Config::new("libiwasm")
        .generator("Unix Makefiles")
        .define("CMAKE_BUILD_TYPE", "Release")
        .no_build_target(true)
        .build();
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("build").display()
    );
    println!("cargo:rustc-link-lib=iwasm");
    println!("cargo:rustc-link-lib=vmlib");

    let bindings = bindgen::Builder::default()
        .header("wasm-micro-runtime/core/iwasm/include/wasm_export.h")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
