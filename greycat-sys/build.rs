use std::process::Command;

fn main() {
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-arg=-Wl,-undefined");
        println!("cargo:rustc-link-arg=-Wl,dynamic_lookup");
    }

    Command::new("greycat")
        .arg("install")
        .output()
        .expect("unable to run: greycat install");

    let bindings = bindgen::Builder::default()
        .header("lib/std/include/greycat.h")
        .allowlist_item("gc_.*")
        .prepend_enum_name(false)
        .anon_fields_prefix("__")
        .use_core()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/generated.rs")
        .expect("Couldn't write bindings!");
}
