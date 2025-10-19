use greycat::prelude::*;

#[greycat_fn]
fn read_file(path: GcString) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path.as_str())
}
