use crate::raytracer::common::Ray;
use crate::raytracer::common::Vec4;
use ndarray::{arr1, arr2, Array1, Array2};

/**
 * Transformation is from pixel-coordinates to world-coordinates.
 * Origin is in world-coordinates.
 * Resolution in pixels.
 *
 */
pub struct Camera {
    pub resolution_x: u32,
    pub resolution_y: u32,
    pub origin: Array1<f64>,
    transformation: Array2<f64>,
}

/**
 *  Transform image pixel (i,j) to image plane coordinates (u, v).
 */
fn compute_transformation(
    resolution_x: u32,
    resolution_y: u32,
    u: Array1<f64>,
    v: Array1<f64>,
    w: Array1<f64>,
    origin: Array1<f64>,
    half_width: f64,
    half_height: f64,
) -> Array2<f64> {
    let spacing = arr1(&[
        half_width * 2.0 / resolution_x as f64,
        half_height * 2.0 / resolution_y as f64,
        100.0 as f64,
    ]);

    // Lower-left corner is the image-plane's origin
    // (-hw, -hl, -1.0)
    let to_image_plane = arr2(&[
        [spacing[0], 0.0, 0.0, -half_width],
        [0.0, spacing[1], 0.0, -half_height],
        [0.0, 0.0, spacing[2], -1.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    let to_world = arr2(&[
        [u[0], v[0], w[0], origin[0]],
        [u[1], v[1], w[1], origin[1]],
        [u[2], v[2], w[2], origin[2]],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    let flip_y = arr2(&[
        [1.0, 0.0, 0.0, 0.0],
        [0.0, -1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    to_world.dot(&flip_y.dot(&to_image_plane))
}

impl Camera {
    pub fn new(
        vertical_fov: f64,
        resolution_x: u32,
        resolution_y: u32,
        origin: Array1<f64>,
        lookat: Array1<f64>,
        up: Array1<f64>,
    ) -> Camera {
        let theta = vertical_fov * std::f64::consts::PI / 180.0;
        let aspect = resolution_x as f64 / resolution_y as f64;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        // Camera orthonormal basis
        let w = Vec4::normalize(origin.clone() - lookat.clone());
        let u = Vec4::normalize(Vec4::cross(up.clone(), w.clone()));
        let v = Vec4::cross(w.clone(), u.clone());

        let transformation = compute_transformation(
            resolution_x,
            resolution_y,
            u.clone(),
            v.clone(),
            w.clone(),
            origin.clone(),
            half_width,
            half_height,
        );

        Camera {
            resolution_x,
            resolution_y,
            origin,
            transformation,
        }
    }

    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        let point_pixels = arr1(&[x, y, 0.0, 1.0]);
        let point_world = self.get_transformation().dot(&point_pixels);

        Ray {
            origin: self.origin.clone(),
            direction: point_world - self.origin.clone(),
        }
    }

    pub fn get_transformation(&self) -> Array2<f64> {
        self.transformation.clone()
    }
}
