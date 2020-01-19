use crate::raytracer::common::Ray;
use crate::raytracer::common::Vec4;
use ndarray::{arr1, arr2, Array1, Array2};
use rand::Rng;

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
    camera_to_world: Array2<f64>,
    lens_radius: f64,
}

/**
 *  Transform image pixel (i,j) to image plane coordinates (u, v).
 */
fn compute_image_to_world(
    resolution_x: u32,
    resolution_y: u32,
    half_width: f64,
    half_height: f64,
    focus_dist: f64,
    to_world: &Array2<f64>,
) -> Array2<f64> {
    let spacing = arr1(&[
        half_width * 2.0 / resolution_x as f64,
        half_height * 2.0 / resolution_y as f64,
    ]);

    // Lower-left corner is the image-plane's origin
    // (-hw, -hl, -1.0)
    let to_image_plane = arr2(&[
        [spacing[0], 0.0, 0.0, -half_width],
        [0.0, spacing[1], 0.0, -half_height],
        [0.0, 0.0, 1.0, -focus_dist],
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

/**
 *  Transform image pixel (i,j) to image plane coordinates (u, v).
 */
fn compute_camera_to_world(
    u: Array1<f64>,
    v: Array1<f64>,
    w: Array1<f64>,
    origin: Array1<f64>,
) -> Array2<f64> {

    let to_world = arr2(&[
        [u[0], v[0], w[0], origin[0]],
        [u[1], v[1], w[1], origin[1]],
        [u[2], v[2], w[2], origin[2]],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    to_world
}


pub fn random_in_unit_disk() -> Array1<f64> {
    let mut p = arr1(&[std::f64::MAX, std::f64::MAX]);
    let mut rng = rand::thread_rng();
    let min = -1.0;
    let max = 1.0;

    while p.dot(&p) >= 1.0 {
        p[0] = rng.gen_range(min, max);
        p[1] = rng.gen_range(min, max);
    }

    arr1(&[p[0], p[1], 0.0, 1.0])
}


impl Camera {
    pub fn new(
        vertical_fov: f64,
        resolution_x: u32,
        resolution_y: u32,
        origin: Array1<f64>,
        lookat: Array1<f64>,
        up: Array1<f64>,
        aperture: f64,
    ) -> Camera {
        let lens_radius = aperture / 2.0;
        let focus_dist = Vec4::l2_norm((origin.clone() - lookat.clone()).view());
        let theta = vertical_fov * std::f64::consts::PI / 180.0;
        let aspect = resolution_x as f64 / resolution_y as f64;
        let half_height = focus_dist * (theta / 2.0).tan();
        let half_width = aspect * half_height;

        // Camera orthonormal basis
        let w = Vec4::normalize(origin.clone() - lookat.clone());
        let u = Vec4::normalize(Vec4::cross(up.clone(), w.clone()));
        let v = Vec4::cross(w.clone(), u.clone());

        let camera_to_world = compute_camera_to_world(
            u.clone(),
            v.clone(),
            w.clone(),
            origin.clone(),
        );

        let transformation = compute_image_to_world(
            resolution_x,
            resolution_y,
            half_width,
            half_height,
            focus_dist,
            &camera_to_world,
        );

        Camera {
            resolution_x,
            resolution_y,
            origin,
            transformation,
            lens_radius,
            camera_to_world,
        }
    }

    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        let point_pixels = arr1(&[x, y, 0.0, 1.0]);
        let point_world = self.get_transformation().dot(&point_pixels);

        if self.lens_radius > 0.0 {
            let rd = self.camera_to_world.dot(&(self.lens_radius * random_in_unit_disk()));
            Ray {
                origin: rd.clone(),
                direction: Vec4::normalize(point_world - rd.clone()),
            }
        }
        else {
            Ray {
                origin: self.origin.clone(),
                direction: Vec4::normalize(point_world - self.origin.clone()),
            }
        }
    }

    pub fn get_transformation(&self) -> Array2<f64> {
        self.transformation.clone()
    }
}
