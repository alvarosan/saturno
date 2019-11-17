pub mod actor;
pub mod camera;
pub mod common;
pub mod common_testing;
pub mod external;

pub struct Image {
    pub width: u32,
    pub height: u32,
    data: Vec<u8>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        // Allocate for 4C (RGBA)
        let size = width as usize * height as usize * 4;
        let mut data: Vec<u8> = Vec::with_capacity(size);
        data.resize(size, 0);
        Image {
            width,
            height,
            data,
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get_pixel(&mut self, index: usize) -> (u32, u32, Option<&mut u8>) {
        // index = y * width + x
        let y = index / self.width as usize;
        let x = index - y as usize * self.width as usize;
        let pixel = self.data.get_mut(index);

        (x as u32, y as u32, pixel)
    }
}

pub mod canvas {
    extern crate rand;

    use rand::Rng;

    use crate::raytracer::actor::Hit;
    use crate::raytracer::actor::RayTraceable;
    use crate::raytracer::camera::Camera;
    use crate::raytracer::common::Ray;
    use crate::raytracer::Image;
    use ndarray::{arr1, Array1};
    use std::vec::Vec;

    pub struct Canvas {
        pub width: u32,
        pub height: u32,
        pub actors: Vec<Box<dyn RayTraceable>>,
        pub samples: u32,
    }

    impl Canvas {
        /**
         *  Compute the background color based on the ray direction.
         *  Use LERP (linear interpolation), to generate a gradient on the
         *  y-direction (similar to front-to-back blending).
         */
        fn background_color(&self, ray: &Ray) -> Array1<f64> {
            let dir = ray.direction.clone();
            let param_y: f64 = 0.5 * (dir[1] + 1.0);

            let white = arr1(&[0.8, 0.8, 0.8, 0.9]);
            let blue = arr1(&[0.1, 0.2, 0.65, 0.9]);
            let color = ((1.0 - param_y) * white + param_y * blue) * 255 as f64;

            color
        }

        fn cast_rays(&self, ray: &Ray) -> Array1<f64> {
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
            let mut color = arr1(&[0.0, 0.0, 0.0, 0.0]);

            for actor in self.actors.iter() {
                if actor.is_hit(&ray, 0.0, closest_so_far, current_hit) {
                    hit_anything = true;
                    closest_so_far = current_hit.t;
                    color = actor.render(&current_hit);
                }
            }

            if !hit_anything {
                color = self.background_color(&ray);
            }

            color
        }

        pub fn render_scene(&self) -> Image {
            let mut image = Image::new(self.width, self.height);

            let camera = Camera::new(
                arr1(&[-2.0, -1.0, -1.0, 1.0]),
                arr1(&[2.0, 1.0, -1.0, 1.0]),
                self.width,
                self.height,
                arr1(&[0.0, 0.0, 0.0, 1.0]),
            );

            // TODO only create it if samples > 1.
            let mut rng = rand::thread_rng();

            for i in 0..image.size() {
                let (x, y, pixel) = image.get_pixel(i);
                let mut color = arr1(&[0.0, 0.0, 0.0, 0.0]);

                // TODO review why the statement below produces weird results...
                // for i in 0..=number_samples {
                for i in 0..self.samples {
                    let mut x_final = x as f64;
                    let mut y_final = y as f64;

                    if i > 0 {
                        x_final = x as f64 + rng.gen_range(0.0, 0.999999);
                        y_final = y as f64 + rng.gen_range(0.0, 0.999999);
                    }

                    let ray = camera.get_ray(x_final, y_final);
                    color = color + self.cast_rays(&ray);
                }

                color = color / self.samples as f64;

//                *pixel = image::Rgba::<u8>([
//                    (color[0]) as u8,
//                    (color[1]) as u8,
//                    (color[2]) as u8,
//                    255,
//                ]);
            }
            image
        }
    }

}
