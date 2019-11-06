use crate::raytracer::common::Ray;
use ndarray::{arr1, arr2, Array1, Array2};

/**
 * Points are in world coordinates [u, v] (e.g. origin,
 * image plane's lower_left and upper_right corners).
 *
 * Resolution in pixels.
 *
 */
pub struct Camera {
    pub lower_left: Array1<f64>,
    pub upper_right: Array1<f64>,
    pub resolution_x: u32,
    pub resolution_y: u32,
    pub origin: Array1<f64>,
    transformation: Array2<f64>,
}

/**
 *  Transform image pixel (i,j) to image plane coordinates (u, v).
 */
fn compute_transformation(
    lower_left: Array1<f64>,
    upper_right: Array1<f64>,
    resolution_x: u32,
    resolution_y: u32,
) -> Array2<f64> {
    let range = upper_right - lower_left.clone();
    let steps: f64 = 100.0;

    let spacing = arr1(&[
        range[0] / resolution_x as f64,
        range[1] / resolution_y as f64,
        range[2] / steps as f64,
    ]);

    let transf = arr2(&[
        [spacing[0], 0.0, 0.0, lower_left[0]],
        [0.0, spacing[1], 0.0, lower_left[1]],
        [0.0, 0.0, spacing[2], lower_left[2]],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    let flip_y = arr2(&[
        [1.0, 0.0, 0.0, 0.0],
        [0.0, -1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    flip_y.dot(&transf)
}

impl Camera {
    pub fn new(
        lower_left: Array1<f64>,
        upper_right: Array1<f64>,
        resolution_x: u32,
        resolution_y: u32,
        origin: Array1<f64>,
    ) -> Camera {
        let transformation = compute_transformation(
            lower_left.clone(),
            upper_right.clone(),
            resolution_x,
            resolution_y,
        );

        Camera {
            lower_left,
            upper_right,
            resolution_x,
            resolution_y,
            origin,
            transformation,
        }
    }

    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        let point_pixels = arr1(&[x, y, 0.0, 1.0]);
        let point_image_plane = self.get_transformation().dot(&point_pixels);

        Ray {
            origin: self.origin.clone(),
            direction: point_image_plane - self.origin.clone(),
        }
    }

    pub fn get_transformation(&self) -> Array2<f64> {
        self.transformation.clone()
    }
}
