use std::env;
use std::path::Path;
use std::path::PathBuf;

pub(crate) fn project_root() -> PathBuf {
    Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
