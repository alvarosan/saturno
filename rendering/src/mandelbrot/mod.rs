extern crate image;
extern crate ndarray;
extern crate num;

use ndarray::{arr1, arr2};
use ndarray::{Array1, Array2};
use num::complex::Complex64;

mod lut;
use crate::mandelbrot::lut::rainbow;

pub fn compute(
    max_iter: u32,
    image_size: Array1<u32>,
    range_x: Array1<f64>,
    range_y: Array1<f64>,
) -> image::RgbaImage {
    let step_x: f64 = (range_x[1] - range_x[0]) / image_size[0] as f64;
    let step_y: f64 = (range_y[1] - range_y[0]) / image_size[1] as f64;

    let mut image = image::RgbaImage::new(image_size[0], image_size[1]);
    let image_to_complex =
        create_transformation(step_x, step_y, range_x[0], range_y[0]);

    let black = image::Rgba::<u8>([0, 0, 0, 255]);
    let lut = rainbow(max_iter);

    println!(
        "> Creating Mandelbrot frame {}x{}...",
        image_size[0], image_size[1]
    );
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let pixel_coord = arr1(&[x as f64, y as f64, 1.0]);
        let complex = image_to_complex.dot(&pixel_coord);

        let iter = mandelbrot(complex[0], complex[1], max_iter as u32);

        if iter < max_iter {
            *pixel = lut[iter as usize];
        } else {
            *pixel = black;
        }
    }

    // Rust idiom for `return image;`
    image
}

fn mandelbrot(x: f64, y: f64, max_iter: u32) -> u32 {
    let c = Complex64::new(x, y);
    let mut i: u32 = 0;
    //let mut z = c.clone();
    // Just start with 0.. more elegant.
    let mut z = Complex64::new(0., 0.);

    // Recompute a new complex number z, as long as it remains
    // within a certain limit radius for N iterations (e.g. if it
    // remains within the radius for N iterations, it would be
    // considered within the stable region).
    while z.norm_sqr() < 4.0 && i < max_iter {
        z = z * z + c;
        i += 1;
    }

    i
}

/**
 * Creates transformation from image coordinates to complex plane.
 */
fn create_transformation(
    x_spacing: f64,
    y_spacing: f64,
    x_min: f64,
    y_min: f64,
) -> Array2<f64> {
    arr2(&[
        [x_spacing, 0.0, x_min],
        [0.0, y_spacing, y_min],
        [0.0, 0.0, 1.0],
    ])
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
