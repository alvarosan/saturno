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
        fn image_to_ndc(&self) -> Array2<f64> {
            // image pixel (i,j) to ndc (image plane coords) (u, v)
            // then use u,v to define the ray direction (ray origin always
            // the same)
            //
            // Finally fill up the background fucntion, which colors
            // based on the ray direction.
            //
            let camera_center = arr1(&[0.0, 0.0, 0.0, 1.0]);
            let lower_left_ndc = arr1(&[-2.0, -1.0, -1.0, 1.0]);
            let upper_right_ndc = arr1(&[2.0, 1.0, -1.0, 1.0]);
            let range = upper_right_ndc - lower_left_ndc.clone();

            let spacing = arr1(&[
                range[0] / self.width as f64,
                range[1] / self.height as f64,
            ]);

            arr2(&[
                [spacing[0], 0.0, 0.0, lower_left_ndc[0]],
                [0.0, spacing[1], 0.0, lower_left_ndc[1]],
                [0.0, 0.0, 1.0, lower_left_ndc[2]],
                [0.0, 0.0, 0.0, 1.0],
            ])
        }

        pub fn background_color(&self,
            ray: &crate::raytracer::ray::Ray,
        ) -> image::Rgba::<u8> {
            let dir = ray.direction.clone();
            let param_y: f64 = 0.5 * (dir[1] + 1.0);

            let white = arr1(&[1.0, 1.0, 1.0]);
            let blue = arr1(&[0.5, 0.7, 1.0]);
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
            let blue = image::Rgba::<u8>([0, 0, 255, 255]);

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

                ray.origin = point_ndc - ray.origin.clone();
                ray.origin.dot(&ray.origin).sqrt()

                *pixel = self.background_color(&ray); 
            }
            image
        }
    }

}
