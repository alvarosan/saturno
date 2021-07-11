use std::fs::create_dir;
use std::path::PathBuf;
use crate::raytracer::image::Image;
use crate::raytracer::image::compute_buffer_difference;

extern crate image;
use image::open;
use image::RgbaImage;

pub fn init_image_testing() -> PathBuf {
    let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_path.push("testing/test_result");

    if !test_path.exists() {
        let _res = create_dir(&test_path);
    }
    
    let mut diff_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    diff_path.push("testing/diff");
    
    if !diff_path.exists() {
        let _res = create_dir(&diff_path);
    }

    test_path
}

pub fn equals_to_baseline(image: Image, path: PathBuf, threshold: f32) {
    
    let mut diff_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    diff_path.push("testing/diff");
    diff_path.push(path.clone().file_name().unwrap());
    
    let mut baseline_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    baseline_path.push("testing/baselines");
    baseline_path.push(path.clone().file_name().unwrap());
        
    let baseline = match open(baseline_path) {
        Ok(baseline) => baseline.to_rgba(),
        _ => panic!(String::from("Failed"))
    };

    let diff_error = match compute_buffer_difference(&Image::from_vec(baseline.width(),
        baseline.height(), 4, baseline.to_vec()), &image, threshold) {
        Ok(diff_error) => diff_error,
        Err(message) => panic!(message)
    };

    let image_diff = RgbaImage::from_raw(image.width, image.height, diff_error.0).unwrap();
    let _result = image_diff.save(diff_path);

    println!("error: {}", diff_error.1);
    let error = diff_error.1;
    if error > 0.0 {
        let mut message = String::from("Image is too different from Baseline image (error = ");
        message.push_str(error.to_string().as_str());
        message.push_str(" ).");

        panic!(message);
    }
}
