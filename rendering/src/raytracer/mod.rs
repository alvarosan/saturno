pub mod actor;
pub mod common;
pub mod common_testing;

pub mod canvas {
    use crate::raytracer::actor::Hit;
    use crate::raytracer::actor::RayTraceable;
    use crate::raytracer::common::Ray;
    use ndarray::{arr1, arr2, Array2};
    use std::vec::Vec;

    extern crate image;

    pub struct Canvas {
        pub width: u32,
        pub height: u32,
        pub actors: Vec<Box<dyn RayTraceable>>,
    }

    impl Canvas {
        /**
         *  Transform image pixel (i,j) to image plane coordinates (u, v).
         */
        fn image_to_ndc(&self) -> Array2<f64> {
            let lower_left_ndc = arr1(&[-2.0, -1.0, -1.0, 1.0]);
            let upper_right_ndc = arr1(&[2.0, 1.0, -1.0, 1.0]);
            let range = upper_right_ndc - lower_left_ndc.clone();
            let steps: f64 = 100.0;

            let spacing = arr1(&[
                range[0] / self.width as f64,
                range[1] / self.height as f64,
                range[2] / steps as f64,
            ]);

            let transf = arr2(&[
                [spacing[0], 0.0, 0.0, lower_left_ndc[0]],
                [0.0, spacing[1], 0.0, lower_left_ndc[1]],
                [0.0, 0.0, spacing[2], lower_left_ndc[2]],
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

        /**
         *  Compute the background color based on the ray direction.
         *  Use LERP (linear interpolation), to generate a gradient on the
         *  y-direction (similar to front-to-back blending).
         */
        fn background_color(&self, ray: &Ray) -> image::Rgba<u8> {
            let dir = ray.direction.clone();
            let param_y: f64 = 0.5 * (dir[1] + 1.0);

            let white = arr1(&[0.8, 0.8, 0.8]);
            let blue = arr1(&[0.1, 0.2, 0.65]);
            let color = ((1.0 - param_y) * white + param_y * blue) * 255 as f64;

            // TODO Hack, using the alpha channel as depth buffer
            image::Rgba::<u8>([
                color[0] as u8,
                color[1] as u8,
                color[2] as u8,
                254,
            ])
        }

        pub fn render_scene(&self) -> image::RgbaImage {
            let mut image = image::RgbaImage::new(self.width, self.height);
            let transf = self.image_to_ndc();

            for (x, y, pixel) in image.enumerate_pixels_mut() {
                let point_image = arr1(&[x as f64, y as f64, 0.0, 1.0]);
                let point_ndc = transf.dot(&point_image);

                // TODO Add default values, perhaps add a vec3, vec4 classes
                let mut ray = Ray {
                    // Camera center is (0, 0, 0)
                    origin: arr1(&[0.0, 0.0, 0.0, 1.0]),
                    direction: arr1(&[1.0, 1.0, 1.0, 0.0]),
                };
                ray.direction = point_ndc - ray.origin.clone();

                // Traverse the vector of RayTraceable instances, and keep track
                // of the closest hit (e.g. closest to the camera hence, not
                // occluded). The closest (t), becomes the maximum depth t we
                // willing to accept as a hit in the following actors.
                let mut hit_anything = false;
                let mut closest_so_far = 999.0;
                let current_hit = &mut Hit {
                    t: 0.0,
                    point: arr1(&[0.0, 0.0, 0.0, 1.0]),
                    normal: arr1(&[0.0, 0.0, 0.0, 0.0]),
                };
                for actor in self.actors.iter() {
                    if actor.is_hit(&ray, 0.0, closest_so_far, current_hit) {
                        hit_anything = true;
                        closest_so_far = current_hit.t;
                        *pixel = actor.render(&current_hit);
                    }
                }

                if !hit_anything {
                    *pixel = self.background_color(&ray);
                }
            }
            image
        }
    }

}
