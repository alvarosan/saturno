pub mod actor;
pub mod camera;
pub mod common;
pub mod common_testing;
pub mod image;
pub mod material;
pub mod scenes;

pub mod canvas {
    extern crate rand;

    use rand::Rng;

    use crate::raytracer::actor::Hit;
    use crate::raytracer::actor::Hittable;
    use crate::raytracer::actor::HittableList;
    use crate::raytracer::actor::RayTraceable;
    use crate::raytracer::camera::Camera;
    use crate::raytracer::common::Ray;
    use crate::raytracer::common::Vec4;
    use crate::raytracer::image::Image;
    use crate::raytracer::image::Pixel;
    use ndarray::{arr1, Array1};
    use rayon::prelude::*;
    use std::vec::Vec;

    pub struct Canvas {
        pub width: u32,
        pub height: u32,
        pub world: HittableList,
        pub samples: u32,
        camera: Camera,
        image: Image,
    }

    impl Canvas {
        pub fn new(
            width: u32,
            height: u32,
            actors: Vec<Box<dyn RayTraceable>>,
            samples: u32,
            camera: Camera,
        ) -> Canvas {
            let world = HittableList::new(actors);
            let image = Image::new(width, height, 4);

            Canvas {
                width,
                height,
                world,
                samples,
                camera,
                image,
            }
        }

        pub fn grab_frame(&self) -> Image {
            self.image.clone()
        }

        /**
         *  Compute the background color based on the ray direction.
         *  Use LERP (linear interpolation), to generate a gradient on the
         *  y-direction (similar to front-to-back blending).
         */
        fn background_color(&self, ray: &Ray) -> Array1<f64> {
            let dir = Vec4::normalize(ray.direction.clone());
            let param_y: f64 = 0.5 * (dir[1] + 1.0);

            //let white = arr1(&[0.8, 0.8, 0.8, 0.9]);
            //let blue = arr1(&[0.1, 0.2, 0.65, 0.9]);
            let white = arr1(&[1.0, 1.0, 1.0, 1.0]);
            let blue = arr1(&[0.5, 0.7, 1.0, 1.0]);
            let color = (1.0 - param_y) * white + param_y * blue;

            color
        }

        fn cast_rays(&self, ray: &Ray, depth: u32) -> Array1<f64> {
            let current_hit = &mut Hit::new();

            // Some of the reflected rays hit the object they are reflecting
            // off of not at exactly t=0, but instead at t=-0.0000001 or
            // t=0.00000001 or whatever floating point approximation the (sphere)
            // intersector gives us. So we need to ignore hits very near zero and
            // we do this by raising the minimum to 0.001.
            if self.world.is_hit(ray, 0.0001, std::f64::MAX, current_hit) {
                let mut attenuation = arr1(&[0.0, 0.0, 0.0, 1.0]);
                let mut scattered = Ray::new(
                    arr1(&[0.0, 0.0, 0.0, 1.0]),
                    arr1(&[0.0, 0.0, 0.0, 0.0]),
                );

                if current_hit.material.scatter(
                    &ray,
                    &current_hit,
                    &mut attenuation,
                    &mut scattered,
                    depth,
                ) {
                    return attenuation * self.cast_rays(&scattered, depth + 1);
                } else {
                    return current_hit.material.color_noscatter(&current_hit);
                }
            } else {
                return self.background_color(&ray);
            }
        }

        pub fn render_scene_rayon(&mut self) {
            let mut rendered_data = self.image.data.clone();
            rendered_data
                .par_iter_mut()
                .enumerate()
                .for_each(&self.render_pixel());
            self.image.data = rendered_data;
        }

        pub fn render_scene(&mut self) {
            let mut rendered_data = self.image.data.clone();
            rendered_data
                .iter_mut()
                .enumerate()
                .for_each(self.render_pixel());
            self.image.data = rendered_data;
        }

        fn render_pixel(
            &mut self,
        ) -> Box<dyn Fn((usize, &mut Pixel<u8>)) -> () + '_ + Sync> {
            return Box::new(move |(index, pixel)| {
                let (x, y) = Image::pixel_coordinate(self.image.width, index);
                let mut color = arr1(&[0.0, 0.0, 0.0, 0.0]);

                color = self.compute_samples(color, x, y);
                color = color / self.samples as f64;

                self.gamma_correct(&mut color, 2.0);
                color = color * 255.0;

                pixel.data =
                    [color[0] as u8, color[1] as u8, color[2] as u8, 255];
            });
        }

        fn compute_samples(
            &self,
            mut color: Array1<f64>,
            x: u32,
            y: u32,
        ) -> Array1<f64> {
            let mut rng = rand::thread_rng();

            // TODO review why the statement below produces weird results...
            // for i in 0..=number_samples {
            for sample in 0..self.samples {
                let mut x_final = x as f64;
                let mut y_final = y as f64;

                if sample > 0 {
                    x_final = x as f64 + rng.gen_range(0.0, 0.999999);
                    y_final = y as f64 + rng.gen_range(0.0, 0.999999);
                }

                let ray = self.camera.get_ray(x_final, y_final);

                color = color + self.cast_rays(&ray, 1);
            }
            color
        }

        fn gamma_correct(&self, color: &mut Array1<f64>, gamma: f64) {
            color.mapv_inplace(|x| x.powf(1.0 / gamma));
        }
    }
}
