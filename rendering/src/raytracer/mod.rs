pub mod actor;
pub mod camera;
pub mod common;
pub mod common_testing;
pub mod external;
pub mod material;
pub mod scenes;

/**
 * Rust does not yet support structs with generic variable-lenght arrays. So,
 * for now, only 4C (RGBA) supported.
 *
 * https://medium.com/@iBelieve/rust-structs-with-generic-variable-length-arrays-7490b68499ea
 */
#[derive(Clone, Copy)]
pub struct Pixel<T> {
    pub data: [T; 4]
}

//////////////////////////////////////////////
//// Pixel-compoennt-iterator
//// Based on:
//// https://stackoverflow.com/questions/30218886/how-to-implement-iterator-and-intoiterator-for-a-simple-struct
////
//struct PixelIntoIterator {
//    pixel: Pixel<u8>,
//    index: usize,
//}
//
//impl IntoIterator for Pixel<u8> {
//    type Item = u8;
//    type IntoIter = PixelIntoIterator;
//
//    fn into_iter(self) -> Self::IntoIter {
//        PixelIntoIterator {
//            pixel: self,
//            index: 0,
//        }
//    }
//}
//
//impl Iterator for PixelIntoIterator {
//    type Item = u8;
//
//    fn next(&mut self) -> Option<u8> {
//        let result = match self.index {
//            0 => self.pixel.data[0],
//            1 => self.pixel.data[1],
//            2 => self.pixel.data[2],
//            3 => self.pixel.data[3],
//            _ => return None,
//        };
//
//        self.index += 1;
//        Some(result)
//    }
//}
//////////////////////////////////////////////
//
//
//

#[derive(Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub chan: u32,
    pub data: Vec<Pixel<u8>>,
}
//
//struct ImageIntoIterator {
//    image: Image,
//    index: usize,
//}
//
//impl IntoIterator for Image {
//    type Item = Pixel<u8>;
//    type IntoIter = ImageIntoIterator;
//
//    fn into_iter(self) -> Self::IntoIter {
//        ImageIntoIterator {
//            image: self,
//            index: 0,
//        }
//    }
//}
//
//impl Iterator for ImageIntoIterator {
//    type Item = Pixel<u8>;
//
//    fn next(&mut self) -> Option<Pixel<u8>> {
//
//        if self.index >= self.image.size() {
//            return None;
//        }
//
//        let result = self.image.data[self.index];
//        self.index += 1;
//        Some(result)
//    }
//}

impl Image {
    pub fn new(width: u32, height: u32, chan: u32) -> Image {
        let size = width as usize * height as usize;
        let mut data: Vec<Pixel<u8>> = Vec::with_capacity(size);
        data.resize(size, Pixel { data: [0, 0, 0, 0] as [u8; 4] });
        Image {
            width,
            height,
            chan,
            data,
        }
    }

    pub fn size(&self) -> usize {
        self.data.len() as usize
    }

    pub fn pixel_coordinate(width: u32, index: usize) -> (u32, u32) {
        // index = y * stride + x
        let stride = width as usize;
        let y = index / stride;
        let x = index - y as usize * stride;

        (x as u32, y as u32)
    }

    pub fn get_pixel_coordinate(&self, index: usize) -> (u32, u32) {
        // index = y * stride + x
        let stride = self.width as usize;
        let y = index / stride;
        let x = index - y as usize * stride;

        (x as u32, y as u32)
    }

    pub fn get_value(&self, x: u32, y: u32, c: u32) -> u8 {
        let index = y * self.width + x;
        let pixel = &self.data[index as usize];

        pixel.data[c as usize]
    }

    pub fn set_pixel(&mut self, index: usize, color: [u8; 4]) {
        self.data[index] = Pixel { data: color };
    }
}

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
    use crate::raytracer::Image;
    use ndarray::{arr1, Array1};
    use std::vec::Vec;
    use rayon::prelude::*;


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

        pub fn render_scene(&mut self) {

            //for index in 0..image.size() {
            //for (index, pixel) in image.data.iter_mut().enumerate() {
            // TODO:  Seems to be trying to still use a regular iter
            // https://www.reddit.com/r/rust/comments/ak5i6f/making_enumerate_happy_with_a_parallel_iterator/
            //
            // Also looks like  enumerate() is not available for ParallelIterator but only for
            // IndexedParallelIterator , which means I need to get a way
            // to get a hand on an IndexedParallelIter
            // https://stackoverflow.com/questions/42721458/how-to-satisfy-the-iterator-trait-bound-in-order-to-use-rayon-here
            //for (index, pixel) in image.data.par_iter().enumerate() {

            let mut mydata = self.image.data.clone();
            let image_width = self.image.width;
            mydata.par_iter_mut().enumerate().for_each(|(n, mut pixel)| {
                let index = n;
                let (x, y) = Image::pixel_coordinate(image_width, index);
                let mut color = arr1(&[0.0, 0.0, 0.0, 0.0]);

                color = self.compute_samples(color, x, y);
                color = color / self.samples as f64;

                self.gamma_correct(&mut color, 2.0);
                color = color * 255.0;

                pixel.data = [color[0] as u8, color[1] as u8, color[2] as u8, 255];
//                image.set_pixel(
//                    index,
//                    [color[0] as u8, color[1] as u8, color[2] as u8, 255],
//                );
                });

            self.image.data = mydata;
        }

        fn compute_samples(
            &self,
            mut color: Array1<f64>,
            x: u32,
            y: u32,
        ) -> Array1<f64> {
            let mut random_gen = rand::thread_rng();

            // TODO review why the statement below produces weird results...
            // for i in 0..=number_samples {
            for sample in 0..self.samples {
                let mut x_final = x as f64;
                let mut y_final = y as f64;

                if sample > 0 {
                    x_final = x as f64 + random_gen.gen_range(0.0, 0.999999);
                    y_final = y as f64 + random_gen.gen_range(0.0, 0.999999);
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
