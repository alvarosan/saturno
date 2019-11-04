pub mod mandelbrot;
pub mod raytracer;

#[cfg(test)]
mod tests {
    use crate::raytracer::canvas::Canvas;
    use crate::raytracer::canvas::Sphere;
    use crate::raytracer::actor::Shading;
    use crate::raytracer::common::Ray;
    use ndarray::arr1;
    use std::fs::File;
    use std::path::Path;

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
        let canvas = Canvas {
            width: 200,
            height: 100,
            actors: vec![],
        };

        let image = canvas.render_scene();
        // TODO Move test output into directory
        // TODO Add image comparisons
        let ref mut out = File::create(&Path::new("background.png")).unwrap();
        let _result = image::ImageRgba8(image).save(out, image::PNG);
        assert_eq!(1.0, 1.0);
    }

    #[test]
    fn render_two_spheres() {
        let mut canvas = Canvas {
            width: 200,
            height: 100,
            actors: vec![],
        };

        let ref mut actors = canvas.actors;
        actors.push(Box::new(Sphere {
            center: arr1(&[0.0, 0.0, -1.0, 1.0]),
            radius: 0.5,
            color: image::Rgba::<u8>([255, 0, 0, 255]),
            shading: Shading::COLOR,
        }));

        actors.push(Box::new(Sphere {
            center: arr1(&[4.0, 1.0, -4.0, 1.0]),
            radius: 0.5,
            color: image::Rgba::<u8>([0, 128, 0, 255]),
            shading: Shading::COLOR,
        }));

        let image = canvas.render_scene();
        let ref mut out = File::create(&Path::new("spheres.png")).unwrap();
        let _result = image::ImageRgba8(image).save(out, image::PNG);
        assert_eq!(1.0, 1.0);
    }

    #[test]
    fn render_sphere_normals() {
        let mut canvas = Canvas {
            width: 200,
            height: 100,
            actors: vec![],
        };

        let ref mut actors = canvas.actors;
        actors.push(Box::new(Sphere {
            center: arr1(&[0.0, 0.0, -1.0, 1.0]),
            radius: 0.5,
            color: image::Rgba::<u8>([255, 0, 0, 255]),
            shading: Shading::NORMALS,
        }));

        let image = canvas.render_scene();
        let ref mut out =
            File::create(&Path::new("sphere_normals.png")).unwrap();
        let _result = image::ImageRgba8(image).save(out, image::PNG);
        assert_eq!(1.0, 1.0);
    }
}
