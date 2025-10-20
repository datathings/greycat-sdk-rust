use std::process::Command;

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

    Command::new("greycat")
        .env("GREYCAT_TARGET", greycat_target(&target_os))
        .arg("install")
        .arg("--force")
        .output()
        .expect("unable to run: greycat install");

    if target_os == "windows" {
        let rel_path = std::path::Path::new("./lib/greycat_sdk_headers.a");
        let lib_path =
            std::fs::canonicalize(rel_path).expect("unable to find lib/greycat_sdk_headers.a");
        println!("cargo:rustc-link-arg=-Wl,--whole-archive");
        println!("cargo:rustc-link-arg={}", lib_path.display());
        println!("cargo:rustc-link-arg=-Wl,--no-whole-archive");
    } else if target_os == "macos" {
        // Quiet MacOS Clang for undefined symbols
        println!("cargo:rustc-link-arg=-Wl,-undefined");
        println!("cargo:rustc-link-arg=-Wl,dynamic_lookup");
    }
}

fn greycat_target(target_os: &str) -> &'static str {
    let arch = &*std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    match target_os {
        "windows" => match arch {
            "x86_64" => "x64-windows",
            _ => panic!("unsupported Windows arch: {}", arch),
        },
        "macos" => match arch {
            "x86_64" => "x64-apple",
            "aarch64" => "arm64-apple",
            _ => panic!("unsupported macOS arch: {}", arch),
        },
        "linux" => match arch {
            "x86_64" => "x64-linux",
            "aarch64" => "arm64-linux",
            _ => panic!("unsupported Linux arch: {}", arch),
        },
        _ => panic!("unsupported target OS: {}", target_os),
    }
}
