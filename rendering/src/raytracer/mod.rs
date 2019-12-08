pub mod actor;
pub mod camera;
pub mod common;
pub mod common_testing;
pub mod external;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub chan: u32,
    pub data: Vec<u8>,
}

impl Image {
    pub fn new(width: u32, height: u32, chan: u32) -> Image {
        // Allocate for 4C (RGBA)
        let size = width as usize * height as usize * chan as usize;
        let mut data: Vec<u8> = Vec::with_capacity(size);
        data.resize(size, 0);
        Image {
            width,
            height,
            chan,
            data,
        }
    }

    pub fn size(&self) -> usize {
        self.data.len() / self.chan as usize
    }

    pub fn get_pixel_coordinate(&mut self, index: usize) -> (u32, u32) {
        // index = y * stride + x
        let stride = self.width as usize;
        let y = index / stride;
        let x = index - y as usize * stride;

        (x as u32, y as u32)
    }

    pub fn get_value(&self, x: u32, y: u32, c: u32) -> u8 {
        let index = (y * self.width + x) * self.chan + c;
        self.data[index as usize]
    }

    pub fn set_pixel(&mut self, index: usize, color: [u8; 4]) {
        //println!(">> Color: {0}, {1}, {2}, {3}", color[0], color[1], color[2], color[3]);

        let j = index * self.chan as usize;
        self.data[j] = color[0];
        self.data[j + 1] = color[1];
        self.data[j + 2] = color[2];
        self.data[j + 3] = color[3];
    }

    pub fn print(&self) {
        for i in 0..self.size() {
            let j = i * 4;
            if i % 800 == 0 {
                println!(
                    ">> data: {0}, {1}, {2}, {3}",
                    self.data[j],
                    self.data[j + 1],
                    self.data[j + 2],
                    self.data[j + 3]
                );
            }
        }
    }
}

pub mod canvas {
    extern crate rand;

    use rand::Rng;

    use crate::raytracer::actor::random_dir_unit_shpere;
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

    pub struct Canvas {
        pub width: u32,
        pub height: u32,
        pub world: HittableList,
        pub samples: u32,
    }

    impl Canvas {
        pub fn new(
            width: u32,
            height: u32,
            actors: Vec<Box<dyn RayTraceable>>,
            samples: u32,
        ) -> Canvas {
            let world = HittableList::new(actors);

            Canvas {
                width,
                height,
                world,
                samples,
            }
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

        fn cast_rays(&self, ray: &Ray) -> Array1<f64> {
            // Traverse the vector of RayTraceable instances, and keep track
            // of the closest hit (e.g. closest to the camera hence, not
            // occluded). The closest (t), becomes the maximum depth t we
            // willing to accept as a hit in the following actors.
            let current_hit = &mut Hit {
                t: 0.0,
                point: arr1(&[0.0, 0.0, 0.0, 1.0]),
                normal: arr1(&[0.0, 0.0, 0.0, 0.0]),
                color: arr1(&[0.0, 0.0, 0.0, 1.0]),
            };


            if self.world.is_hit(ray, 0.0, std::f64::MAX, current_hit) {

                    let target = current_hit.point.clone()
                        + current_hit.normal.clone()
                        + random_dir_unit_shpere();

                    let reflected_ray = Ray::new(
                        current_hit.point.clone(),
                        target - current_hit.point.clone(),
                    );

                    let absorption: f64 = 0.5;
                    //return current_hit.color.clone();
                    return absorption * self.cast_rays(&reflected_ray);
            }
            else {
                return self.background_color(&ray);
            }

        }

        pub fn render_scene(&self) -> Image {
            let mut image = Image::new(self.width, self.height, 4);

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
                let (x, y) = image.get_pixel_coordinate(i);
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

                self.gamma_correct(&mut color, 2.0);
                color = color * 255.0;

                image.set_pixel(
                    i,
                    [color[0] as u8, color[1] as u8, color[2] as u8, 255],
                );
            }
            image
        }

        fn gamma_correct(&self, color: &mut Array1<f64>, gamma: f64) {
            color.mapv_inplace(|x| x.powf(1.0/gamma) );
        }
    }
}
