use std::fs::create_dir;
use std::path::PathBuf;
use crate::raytracer::image::Image;

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


pub fn compute_buffer_difference(buff_a: Vec<u8>, buff_b: Vec<u8>) -> Vec<u8> {
    let mut diff = vec![0; buff_a.len()];
    let channels: usize = 4;
    let mut index: usize = 0;

    loop {
        diff[index] = ((buff_a[index] as i32 - buff_b[index] as i32).abs() * 2)  as u8;
        diff[index+1] = ((buff_a[index+1] as i32 - buff_b[index+1] as i32).abs())  as u8;
        diff[index+2] = ((buff_a[index+2] as i32 - buff_b[index+2] as i32).abs())  as u8;
        diff[index+3] = 255;
        
        index += channels;
        if index >= buff_a.len() {
            break;
        }
    }

    diff
}

pub fn equals_to_baseline(image: Image, path: PathBuf) -> Result<bool, String> {
    
    let mut diff_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    diff_path.push("testing/diff");
    diff_path.push(path.clone().file_name().unwrap());
    
    let mut baseline_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    baseline_path.push("testing/baselines");
    baseline_path.push(path.clone().file_name().unwrap());
        
    let baseline = match open(baseline_path) {
        Ok(baseline) => baseline.to_rgba(),
        _ => return Err(String::from("Failed"))
    };

    if baseline.width() != image.width ||
       baseline.height() != image.height {

        let mut message = String::from("Dimension mismatch: ");
        message.push_str(image.width.to_string().as_str());
        message.push(',');
        message.push_str(image.height.to_string().as_str());

        return Err(message);
    }

    let diff = compute_buffer_difference(baseline.to_vec(), image.as_flat_vec_u8());
    
    let image_diff = RgbaImage::from_raw(image.width, image.height, diff).unwrap();
    let _result = image_diff.save(diff_path);

    Ok(true)
}
