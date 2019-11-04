pub mod common;

pub mod canvas {
    use crate::raytracer::common::Ray;
    use crate::raytracer::actor::Renderable;
    use ndarray::{arr1, arr2, Array2};
    use std::vec::Vec;

    extern crate image;

    pub struct Canvas {
        pub width: u32,
        pub height: u32,
        pub actors: Vec<Box<dyn Renderable>>,
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
        fn background_color(
            &self,
            ray: &Ray,
        ) -> image::Rgba<u8> {
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

                // TODO Add default values, perhaps add a vec3 , vec4 classes
                let mut ray = Ray {
                    // Camera center is (0, 0, 0)
                    origin: arr1(&[0.0, 0.0, 0.0, 1.0]),
                    direction: arr1(&[1.0, 1.0, 1.0, 0.0]),
                };

                ray.direction = point_ndc - ray.origin.clone();

                //let nor = crate::raytracer::common::vec4::normalize(
                //    arr1(&[ray.direction[0], ray.direction[1], ray.direction[2]]));
                //ray.direction = arr1(&[nor[0], nor[1], nor[2], 0.0]);

                *pixel = self.background_color(&ray);

                for actor in self.actors.iter() {
                    // TODO pseudo depht-test using alpha
                    if pixel[3] > 254 {
                        continue;
                    }

                    let sphere_color = actor.render(&ray);
                    if sphere_color[3] == 255 {
                        *pixel = sphere_color;
                    }
                }
            }
            image
        }
    }

}

pub mod actor {
    use ndarray::Array1;
    use crate::raytracer::common::Vec4;
    use crate::raytracer::common::Ray;

    pub enum Shading {
        COLOR,
        NORMALS,
    }

    pub struct Actor<T> {
        pub geometry: T,
    }

    /**
     * Traits in rust are how interfaces are implemented. Depending on their
     * usage, they can be statically or dinamically dispatched.
     */
    pub trait Renderable {
        fn render(&self, ray: &Ray) -> image::Rgba<u8>;
    }

    pub struct Sphere {
        pub center: Array1<f64>,
        pub radius: f64,
        pub color: image::Rgba<u8>,
        pub shading: Shading,
    }

    impl Sphere {
        /**
         * Solving the sphere equation analitically, leads to real solutions
         * (hit front / back) or a complex solution (miss).
         *
         * vec{radius} = vec{Ray} - vec{Center}
         *           X = Y
         *   dot(X, X) = dot(Y, Y)
         *
         * Substitute Ray = Origin + t * Dir and solve for t ...
         *
         * t^2 dot(Dir, Dir) + 2*t*dot(Dir, Orig - Cent) +
         *      dot(Orig-Cent, Orig-Cent) = radius^2
         *
         */
        fn is_hit(&self, ray: &Ray) -> f64 {
            let oc = ray.origin.clone() - self.center.clone();
            let a = ray.direction.dot(&ray.direction);
            let b = 2.0 * oc.dot(&ray.direction);
            let c = oc.dot(&oc) - self.radius * self.radius;
            let discriminant = b * b - 4.0 * a * c;

            if discriminant < 0.0 {
                -1.0
            } else {
                (-b - discriminant.sqrt()) / (2.0 * a)
            }
        }

        /**
         * P - C = Radial Vector
         *
         * Note that the range of the normalized components of the unit normals
         * is [-1.0, 1.0].
         */
        fn compute_normal(&self, point_sphere: &Array1<f64>) -> Array1<f64> {
            let n = point_sphere.clone() - self.center.clone();
            Vec4::normalize(n)
        }
    }

    impl Renderable for Sphere {
        fn render(&self, ray: &Ray) -> image::Rgba<u8> {
            let t = self.is_hit(ray);
            if t > 0.0 {
                match self.shading {
                    crate::raytracer::actor::Shading::COLOR => {
                        return self.color.clone()
                    }
                    crate::raytracer::actor::Shading::NORMALS => {
                        let normal =
                            self.compute_normal(&ray.point_at_parameter(t));

                        // In order to use the normal vectors (i,j,k) as (r,g,b)
                        // they need to be mapped from [-1.0, 1.0] to the
                        // [0.0, 1.0] range.
                        return image::Rgba::<u8>([
                            (255.0 * (normal[0] + 1.0) * 0.5) as u8,
                            (255.0 * (normal[1] + 1.0) * 0.5) as u8,
                            (255.0 * (normal[2] + 1.0) * 0.5) as u8,
                            255,
                        ]);
                    }
                }
            }

            image::Rgba::<u8>([0, 0, 0, 0])
        }
    }

}
