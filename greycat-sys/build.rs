use std::process::Command;

fn main() {
    Command::new("greycat")
        .arg("install")
        .output()
        .expect("unable to run: greycat install");

    // let greycat_headers =
    //     find_greycat_headers("lib/std/include").expect("unable to find greycat headers");

    // eprintln!("{greycat_headers:#?}");

    let bindings = bindgen::Builder::default()
        .header("lib/std/include/greycat.h")
        .clang_arg("-Ilib/std/include")
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

// fn find_greycat_headers(dir: &str) -> std::io::Result<Vec<String>> {
//     let mut headers = Vec::new();
//     collect_greycat_headers(Path::new(dir), &mut headers)?;
//     Ok(headers)
// }

// fn collect_greycat_headers(dir: &Path, headers: &mut Vec<String>) -> std::io::Result<()> {
//     if dir.is_dir() {
//         for entry in std::fs::read_dir(dir)? {
//             let entry = entry?;
//             let path = entry.path();

//             if path.is_dir() {
//                 collect_greycat_headers(&path, headers)?;
//             } else if path.extension().is_some_and(|ext| ext == "h") {
//                 headers.push(path.to_str().expect("invalid filepath").to_string());
//             }
//         }
//     }
//     Ok(())
// }
