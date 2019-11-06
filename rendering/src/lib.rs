pub mod mandelbrot;
pub mod raytracer;

#[cfg(test)]
mod tests {
    use crate::raytracer::actor::Shading;
    use crate::raytracer::actor::Sphere;
    use crate::raytracer::canvas::Canvas;
    use crate::raytracer::common::Ray;
    use crate::raytracer::common_testing::init_image_testing;
    use ndarray::arr1;
    use std::fs::File;

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

        let canvas = Canvas {
            width: 200,
            height: 100,
            actors: vec![],
            samples: 1,
        };

        let image = canvas.render_scene();
        let ref mut out = File::create(&output_path).unwrap();
        let _result = image::ImageRgba8(image).save(out, image::PNG);
        assert_eq!(1.0, 1.0);
    }

    #[test]
    fn render_two_spheres() {
        let mut output_path = init_image_testing();
        output_path.push("render_two_spheres.png");

        let mut canvas = Canvas {
            width: 200,
            height: 100,
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
        let ref mut out = File::create(&output_path).unwrap();
        let _result = image::ImageRgba8(image).save(out, image::PNG);
        assert_eq!(1.0, 1.0);
    }

    #[test]
    fn render_sphere_normals() {
        let mut output_path = init_image_testing();
        output_path.push("render_sphere_normals.png");

        let mut canvas = Canvas {
            width: 200,
            height: 100,
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
        let ref mut out = File::create(&output_path).unwrap();
        let _result = image::ImageRgba8(image).save(out, image::PNG);
        assert_eq!(1.0, 1.0);
    }

    #[test]
    fn render_two_spheres_normals() {
        let mut output_path = init_image_testing();
        output_path.push("render_two_spheres_normals.png");

        let mut canvas = Canvas {
            width: 200,
            height: 100,
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
        let ref mut out = File::create(&output_path).unwrap();
        let _result = image::ImageRgba8(image).save(out, image::PNG);
        assert_eq!(1.0, 1.0);
    }

    #[test]
    fn render_two_spheres_antialiasing() {
        let mut output_path = init_image_testing();
        output_path.push("render_two_spheres_antialiasing.png");

        let mut canvas = Canvas {
            width: 200,
            height: 100,
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
        let ref mut out = File::create(&output_path).unwrap();
        let _result = image::ImageRgba8(image).save(out, image::PNG);
        assert_eq!(1.0, 1.0);
    }
}
