extern crate bindgen;

use cmake;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search={}", env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=whisper");
    println!("cargo:rerun-if-changed=wrapper.h");

    if env::var("WHISPER_DONT_GENERATE_BINDINGS").is_ok() {
        let _: u64 = std::fs::copy(
            "src/bindings.rs",
            env::var("OUT_DIR").unwrap() + "/bindings.rs",
        )
        .expect("Failed to copy bindings.rs");
    } else {
        let bindings = bindgen::Builder::default()
            .header("wrapper.h")
            .clang_arg("-I./whisper.cpp")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate();

        match bindings {
            Ok(b) => {
                let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
                b.write_to_file(out_path.join("bindings.rs"))
                    .expect("Couldn't write bindings!");
            }
            Err(e) => {
                println!("cargo:warning=Unable to generate bindings: {}", e);
                println!("cargo:warning=Using bundled bindings.rs, which may be out of date");
                // copy src/bindings.rs to OUT_DIR
                std::fs::copy(
                    "src/bindings.rs",
                    env::var("OUT_DIR").unwrap() + "/bindings.rs",
                )
                .expect("Unable to copy bindings.rs");
            }
        }
    };

    // stop if we're on docs.rs
    if env::var("DOCS_RS").is_ok() {
        return;
    }

    // build libwhisper.a
    env::set_current_dir("whisper.cpp").expect("Unable to locate a directory");

    let dst = cmake::Config::new(".").define("BUILD_SHARED_LIBS", "off").build();
    // move libwhisper.a to where Cargo expects it (OUT_DIR)
    std::fs::copy(
        format!("{}/lib/static/whisper.lib", dst.display()),
        env::var("OUT_DIR").unwrap() + "/whisper.lib",
    )
    .expect("Failed to copy whisper.lib");

    // std::fs::copy(
    //     format!("{}/build/bin/Debug/whisper.dll", dst.display()),
    //     env::var("OUT_DIR").unwrap() + "/whisper.dll",
    // )
    // .expect("Failed to copy whisper.dll");

    
}
