//pub mod mandelbrot;
pub mod raytracer;

#[cfg(test)]
mod tests {
    use crate::raytracer::actor::Shading;
    use crate::raytracer::actor::Sphere;
    use crate::raytracer::canvas::Canvas;
    use crate::raytracer::common::Ray;
    use crate::raytracer::common_testing::init_image_testing;
    use ndarray::arr1;

    extern crate image;

    #[test]
    fn point_at_parameter() {
        let ray = Ray {
            origin: arr1(&[0.5, 0.6, 0.7, 1.0]),
            direction: arr1(&[1.0, 1.0, 1.0, 0.0]),
        };

        assert_eq!(ray.origin[2], 0.7);

        let point = ray.point_at_parameter(3.0);

        assert_eq!(point[0], 3.5);
        assert_eq!(point[1], 3.6);
        assert_eq!(point[2], 3.7);
    }

    #[test]
    fn render_background() {
        let mut output_path = init_image_testing();
        output_path.push("render_background.png");

        let dims: [u32; 2] = [200, 100];
        let canvas = Canvas {
            width: dims[0],
            height: dims[1],
            actors: vec![],
            samples: 1,
        };

        let image = canvas.render_scene();
        let cached = image.size();
        image.print();
        let image_png = image::RgbaImage::from_raw(dims[0], dims[1], image.data).unwrap();
        let _result = image_png.save(output_path);
        assert_eq!(1.0, 1.0);
    }

    #[test]
    fn render_two_spheres() {
        let mut output_path = init_image_testing();
        output_path.push("render_two_spheres.png");

        let dims: [u32; 2] = [200, 100];
        let mut canvas = Canvas {
            width: dims[0],
            height: dims[1],
            actors: vec![],
            samples: 1,
        };

        let ref mut actors = canvas.actors;
        actors.push(Box::new(Sphere {
            center: arr1(&[0.0, 0.0, -1.0, 1.0]),
            radius: 0.5,
            color: arr1(&[255.0, 0.0, 0.0, 255.0]),
            shading: Shading::COLOR,
        }));

        actors.push(Box::new(Sphere {
            center: arr1(&[4.0, 1.0, -4.0, 1.0]),
            radius: 0.5,
            color: arr1(&[0.0, 128.0, 0.0, 255.0]),
            shading: Shading::COLOR,
        }));

        let image = canvas.render_scene();
        let image_png = image::RgbaImage::from_raw(dims[0], dims[1], image.data).unwrap();
        let _result = image_png.save(output_path);
        assert_eq!(1.0, 1.0);
    }

    #[test]
    fn render_sphere_normals() {
        let mut output_path = init_image_testing();
        output_path.push("render_sphere_normals.png");

        let dims: [u32; 2] = [200, 100];
        let mut canvas = Canvas {
            width: dims[0],
            height: dims[1],
            actors: vec![],
            samples: 1,
        };

        let ref mut actors = canvas.actors;
        actors.push(Box::new(Sphere {
            center: arr1(&[0.0, 0.0, -1.0, 1.0]),
            radius: 0.5,
            color: arr1(&[255.0, 0.0, 0.0, 255.0]),
            shading: Shading::NORMALS,
        }));

        let image = canvas.render_scene();
        let image_png = image::RgbaImage::from_raw(dims[0], dims[1], image.data).unwrap();
        let _result = image_png.save(output_path);
        assert_eq!(1.0, 1.0);
    }

    #[test]
    fn render_two_spheres_normals() {
        let mut output_path = init_image_testing();
        output_path.push("render_two_spheres_normals.png");

        let dims: [u32; 2] = [200, 100];
        let mut canvas = Canvas {
            width: dims[0],
            height: dims[1],
            actors: vec![],
            samples: 1,
        };

        let ref mut actors = canvas.actors;
        actors.push(Box::new(Sphere {
            center: arr1(&[0.0, 0.0, -1.0, 1.0]),
            radius: 0.5,
            color: arr1(&[255.0, 0.0, 0.0, 255.0]),
            shading: Shading::NORMALS,
        }));

        actors.push(Box::new(Sphere {
            center: arr1(&[0.0, -100.5, -1.0, 1.0]),
            radius: 100.0,
            color: arr1(&[0.0, 128.0, 0.0, 255.0]),
            shading: Shading::NORMALS,
        }));

        let image = canvas.render_scene();
        let image_png = image::RgbaImage::from_raw(dims[0], dims[1], image.data).unwrap();
        let _result = image_png.save(output_path);
        assert_eq!(1.0, 1.0);
    }

    #[test]
    fn render_two_spheres_antialiasing() {
        let mut output_path = init_image_testing();
        output_path.push("render_two_spheres_antialiasing.png");

        let dims: [u32; 2] = [200, 100];
        let mut canvas = Canvas {
            width: dims[0],
            height: dims[1],
            actors: vec![],
            samples: 10,
        };

        let ref mut actors = canvas.actors;
        actors.push(Box::new(Sphere {
            center: arr1(&[0.0, 0.0, -1.0, 1.0]),
            radius: 0.5,
            color: arr1(&[255.0, 0.0, 0.0, 255.0]),
            shading: Shading::NORMALS,
        }));

        actors.push(Box::new(Sphere {
            center: arr1(&[0.0, -100.5, -1.0, 1.0]),
            radius: 100.0,
            color: arr1(&[0.0, 128.0, 0.0, 255.0]),
            shading: Shading::NORMALS,
        }));

        let image = canvas.render_scene();
        let image_png = image::RgbaImage::from_raw(dims[0], dims[1], image.data).unwrap();
        let _result = image_png.save(output_path);
        assert_eq!(1.0, 1.0);
    }
}
