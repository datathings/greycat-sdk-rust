use std::path::{Path, PathBuf};

pub fn test_resource(filename: impl AsRef<Path>) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("resources")
        .join(filename.as_ref())
}
