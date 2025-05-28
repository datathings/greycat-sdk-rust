use std::process::Command;

fn main() {
    Command::new("greycat")
        .arg("install")
        .output()
        .expect("unable to run: greycat install");

    let bindings = bindgen::Builder::default()
        .header("lib/std/include/greycat.h")
        .allowlist_item("gc_.*")
        .prepend_enum_name(false)
        .anon_fields_prefix("__")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
