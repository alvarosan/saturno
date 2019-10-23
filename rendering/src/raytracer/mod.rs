pub mod commmon {
    use ndarray::{Array1, ArrayView1};

    pub struct vec4 {
        data: Array1<f64>,
    }

    impl vec4 {
        pub fn x(&self) -> f64 { self.data[0] }
        pub fn y(&self) -> f64 { self.data[1] }
        pub fn z(&self) -> f64 { self.data[2] }
        pub fn w(&self) -> f64 { self.data[3] }
        pub fn r(&self) -> f64 { self.data[0] }
        pub fn g(&self) -> f64 { self.data[1] }
        pub fn b(&self) -> f64 { self.data[2] }
        pub fn a(&self) -> f64 { self.data[3] }

        pub fn normalized(&self) -> vec4 {
            vec4{ data: vec4::normalize(self.data.clone()) }
        }

        fn l2_norm(x: ArrayView1<f64>) -> f64 {
            x.dot(&x).sqrt()
        }

        fn normalize(mut x: Array1<f64>) -> Array1<f64> {
            let norm: f64 = vec4::l2_norm(x.view());
            x.mapv_inplace(|e| e/norm);
            x
        }
    }
}

pub mod ray {
    use ndarray::Array1;

    pub struct Ray {
        pub origin: Array1<f64>,
        pub direction: Array1<f64>,
    }

    impl Ray {
        pub fn point_at_parameter(&self, t: f64) -> Array1<f64> {
            self.origin.clone() + t * self.direction.clone()
        }
    }
}

pub mod canvas {
    use ndarray::{arr1, arr2, Array2};

    extern crate image;

    pub struct Canvas {
        pub width: u32,
        pub height: u32,
    }

    impl Canvas {

        /**
         *  Transform image pixel (i,j) to image plane coordinates (u, v).
         */
        fn image_to_ndc(&self) -> Array2<f64> {
            //let camera_center = arr1(&[0.0, 0.0, 0.0, 1.0]);
            let lower_left_ndc = arr1(&[-2.0, -1.0, -1.0, 1.0]);
            let upper_right_ndc = arr1(&[2.0, 1.0, -1.0, 1.0]);
            let range = upper_right_ndc - lower_left_ndc.clone();

            let spacing = arr1(&[
                               range[0] / self.width as f64,
                               range[1] / self.height as f64,
            ]);

            let transf = arr2(&[
                         [spacing[0], 0.0, 0.0, lower_left_ndc[0]],
                         [0.0, spacing[1], 0.0, lower_left_ndc[1]],
                         [0.0, 0.0, 1.0, lower_left_ndc[2]],
                         [0.0, 0.0, 0.0, 1.0], ]);

            let flip_y = arr2(&[
                              [1.0, 0.0, 0.0, 0.0],
                              [0.0, -1.0, 0.0, 0.0],
                              [0.0, 0.0, 1.0, 0.0],
                              [0.0, 0.0, 0.0, 1.0],  ]);

            flip_y.dot(&transf)
        }

        /**
         *  Compute the background color based on the ray direction.
         *  Use LERP (linear interpolation), to generate a gradient on the
         *  y-direction (similar to front-to-back blending).
         */
        pub fn background_color(&self,
                                ray: &crate::raytracer::ray::Ray,
                                ) -> image::Rgba::<u8> {
            let dir = ray.direction.clone();
            let param_y: f64 = 0.5 * (dir[1] + 1.0);

            let white = arr1(&[0.8, 0.8, 0.8]);
            let blue = arr1(&[0.1, 0.2, 1.0]);
            let color = ((1.0 - param_y) * white + param_y * blue) * 255 as f64;

            image::Rgba::<u8>([
                              color[0] as u8,
                              color[1] as u8,
                              color[2] as u8,
                              255,
            ])
        }

        pub fn render_background(&self) -> image::RgbaImage {
            let mut image = image::RgbaImage::new(self.width, self.height);
            let transf = self.image_to_ndc();

            for (x, y, pixel) in image.enumerate_pixels_mut() {
                let point_image = arr1(&[x as f64, y as f64, 1.0, 1.0]);
                let point_ndc = transf.dot(&point_image);

                // Set Z to where the image plane is located
                println!("Image_p / NDC_p: {} / {}", &point_image, &point_ndc);

                // TODO Add default values, perhaps add a vec3 , vec4 classes
                let mut ray = crate::raytracer::ray::Ray {
                    origin: arr1(&[0.0, 0.0, 0.0, 0.0]),
                    direction: arr1(&[1.0, 1.0, 1.0, 1.0]),
                };

                ray.direction = point_ndc - ray.origin.clone();
                //ray.origin.dot(&ray.origin).sqrt()

                *pixel = self.background_color(&ray);
            }
            image
        }
    }

}
