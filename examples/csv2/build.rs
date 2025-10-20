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

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-arg=-Wl,-undefined");
        println!("cargo:rustc-link-arg=-Wl,dynamic_lookup");
    }
}
