use std::path::PathBuf;

#[cfg(feature = "embed-any")]
pub fn path_for_file(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("files")
        .join(filename)
}
