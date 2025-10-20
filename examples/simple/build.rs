// use std::process::Command;

fn main() {
    // Command::new("greycat")
    //     .arg("install")
    //     .output()
    //     .expect("unable to run: greycat install");

    // Command::new("greycat")
    //     .arg("codegen")
    //     .arg("rust")
    //     .output()
    //     .expect("unable to run: greycat codegen rust");

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    if target_os == "windows" {
        // Only build staticlib for Windows
        println!("cargo:rustc-cfg=crate_type=\"staticlib\"");
    } else if target_os == "macos" {
        // Quiet MacOS Clang for undefined symbols
        println!("cargo:rustc-link-arg=-Wl,-undefined");
        println!("cargo:rustc-link-arg=-Wl,dynamic_lookup");
        // Build cdylib for MacOS
        println!("cargo:rustc-cfg=crate_type=\"cdylib\"");
    } else {
        // Build cdylib for Unix-like systems
        println!("cargo:rustc-cfg=crate_type=\"cdylib\"");
    }
}
