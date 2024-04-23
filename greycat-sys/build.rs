// use std::env::{self, VarError};
// use std::path::PathBuf;

fn main() {
    // let header = match std::env::var("GREYCAT_HEADER") {
    //     Ok(header) => header,
    //     Err(VarError::NotPresent) => match home::home_dir() {
    //         Some(mut home_dir) => {
    //             home_dir.push(".greycat");
    //             home_dir.push("include");
    //             home_dir.push("greycat.h");
    //             home_dir.to_string_lossy().to_string()
    //         }
    //         None => panic!("unable to find greycat.h, missing HOME directory"),
    //     },
    //     Err(e) => panic!("invalid GREYCAT_HEADER env: {e}"),
    // };

    // let bindings = bindgen::Builder::default()
    //     .header(header)
    //     .allowlist_item("gc_.*")
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    //     .generate()
    //     .expect("unable to generate bindings");

    // // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("couldn't write bindings!");
}
