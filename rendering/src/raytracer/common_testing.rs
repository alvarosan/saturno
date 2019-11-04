use std::path::{PathBuf};
use std::fs::{create_dir};

pub fn init_image_testing() -> PathBuf {
    let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_path.push("testing/");

    if !test_path.exists() {
        let _res = create_dir(&test_path);
    }
    
    test_path
}

// TODO Add image comparisons
