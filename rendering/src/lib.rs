//pub mod mandelbrot;
pub mod raytracer;

#[macro_use(s)]
extern crate ndarray;

#[cfg(test)]
mod tests {
    use crate::raytracer::actor::RayTraceable;
    use crate::raytracer::actor::Sphere;
    use crate::raytracer::camera::Camera;
    use crate::raytracer::canvas::Canvas;
    use crate::raytracer::common::Ray;
    use crate::raytracer::common_testing::init_image_testing;
    use crate::raytracer::material::Lambertian;
    use crate::raytracer::material::Primary;
    use crate::raytracer::material::Metal;
    use crate::raytracer::material::Dielectric;
    use crate::raytracer::material::Shading;
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
        let camera = Camera::new(
            90.0,
            dims[0],
            dims[1],
            arr1(&[0.0, 0.0, 0.0, 1.0]),
            arr1(&[0.0, 0.0, -1.0, 1.0]),
            arr1(&[0.0, 1.0, 0.0, 0.0]),
        );
        let canvas = Canvas::new(dims[0], dims[1], vec![], 1, camera);

        let image = canvas.render_scene();
        //image.print();
        let image_png =
            image::RgbaImage::from_raw(dims[0], dims[1], image.data).unwrap();
        let _result = image_png.save(output_path);
        assert_eq!(1.0, 1.0);
    }

    #[test]
    fn render_sphere_color() {
        let mut output_path = init_image_testing();
        output_path.push("render_sphere_color.png");

        let mut actors = vec![];
        actors.push(Box::new(Sphere {
            center: arr1(&[0.0, 0.0, -1.0, 1.0]),
            radius: 0.5,
            material: Box::new(Primary::new(
                arr1(&[1.0, 0.0, 0.0, 1.0]),
                Shading::COLOR,
            )),
        }) as Box<dyn RayTraceable>);

        let dims: [u32; 2] = [200, 100];
        let camera = Camera::new(
            90.0,
            dims[0],
            dims[1],
            arr1(&[0.0, 0.0, 0.0, 1.0]),
            arr1(&[0.0, 0.0, -1.0, 1.0]),
            arr1(&[0.0, 1.0, 0.0, 0.0]),
        );
        let canvas = Canvas::new(dims[0], dims[1], actors, 1, camera);
        let image = canvas.render_scene();
        let image_png =
            image::RgbaImage::from_raw(dims[0], dims[1], image.data).unwrap();
        let _result = image_png.save(output_path);
        assert_eq!(1.0, 1.0);
    }

    #[test]
    fn render_sphere_normals() {
        let mut output_path = init_image_testing();
        output_path.push("render_sphere_normals.png");

        let mut actors = vec![];
        actors.push(Box::new(Sphere {
            center: arr1(&[0.0, 0.0, -1.0, 1.0]),
            radius: 0.5,
            material: Box::new(Primary::new(
                arr1(&[1.0, 0.0, 0.0, 1.0]),
                Shading::NORMALS,
            )),
        }) as Box<dyn RayTraceable>);

        let dims: [u32; 2] = [200, 100];
        let camera = Camera::new(
            90.0,
            dims[0],
            dims[1],
            arr1(&[0.0, 0.0, 0.0, 1.0]),
            arr1(&[0.0, 0.0, -1.0, 1.0]),
            arr1(&[0.0, 1.0, 0.0, 0.0]),
        );
        let canvas = Canvas::new(dims[0], dims[1], actors, 1, camera);
        let image = canvas.render_scene();
        let image_png =
            image::RgbaImage::from_raw(dims[0], dims[1], image.data).unwrap();
        let _result = image_png.save(output_path);
        assert_eq!(1.0, 1.0);
    }

    #[test]
    fn render_antialiasing() {
        let mut output_path = init_image_testing();
        output_path.push("render_antialiasing.png");

        let mut actors = vec![];
        actors.push(Box::new(Sphere {
            center: arr1(&[0.0, 0.0, -1.0, 1.0]),
            radius: 0.5,
            material: Box::new(Primary::new(
                arr1(&[1.0, 0.0, 0.0, 1.0]),
                Shading::NORMALS,
            )),
        }) as Box<dyn RayTraceable>);

        actors.push(Box::new(Sphere {
            center: arr1(&[0.0, -100.5, -1.0, 1.0]),
            radius: 100.0,
            material: Box::new(Primary::new(
                arr1(&[0.0, 0.5, 0.0, 1.0]),
                Shading::NORMALS,
            )),
        }) as Box<dyn RayTraceable>);

        let dims: [u32; 2] = [200, 100];
        let camera = Camera::new(
            90.0,
            dims[0],
            dims[1],
            arr1(&[0.0, 0.0, 0.0, 1.0]),
            arr1(&[0.0, 0.0, -1.0, 1.0]),
            arr1(&[0.0, 1.0, 0.0, 0.0]),
        );
        let canvas = Canvas::new(dims[0], dims[1], actors, 10, camera);
        let image = canvas.render_scene();
        let image_png =
            image::RgbaImage::from_raw(dims[0], dims[1], image.data).unwrap();
        let _result = image_png.save(output_path);
        assert_eq!(1.0, 1.0);
    }

    #[test]
    fn render_diffuse() {
        let mut output_path = init_image_testing();
        output_path.push("render_diffuse.png");

        let mut actors = vec![];
        actors.push(Box::new(Sphere {
            center: arr1(&[0.0, 0.0, -1.0, 1.0]),
            radius: 0.5,
            material: Box::new(Lambertian::new(
                arr1(&[1.0, 0.0, 0.0, 1.0]),
                Shading::NORMALS,
            )),
        }) as Box<dyn RayTraceable>);

        actors.push(Box::new(Sphere {
            center: arr1(&[0.0, -100.5, -1.0, 1.0]),
            radius: 100.0,
            material: Box::new(Lambertian::new(
                arr1(&[1.0, 0.0, 0.0, 1.0]),
                Shading::COLOR,
            )),
        }) as Box<dyn RayTraceable>);

        let dims: [u32; 2] = [200, 100];
        let camera = Camera::new(
            90.0,
            dims[0],
            dims[1],
            arr1(&[0.0, 0.0, 0.0, 1.0]),
            arr1(&[0.0, 0.0, -1.0, 1.0]),
            arr1(&[0.0, 1.0, 0.0, 0.0]),
        );
        let canvas = Canvas::new(dims[0], dims[1], actors, 1, camera);
        let image = canvas.render_scene();
        let image_png =
            image::RgbaImage::from_raw(dims[0], dims[1], image.data).unwrap();
        let _result = image_png.save(output_path);
        assert_eq!(1.0, 1.0);
    }

    #[test]
    fn camera_positionable() {
        let mut output_path = init_image_testing();
        output_path.push("camera_positionable.png");

        let mut actors = vec![];

        actors.push(Box::new(Sphere {
            center: arr1(&[0.0, -100.5, -1.0, 1.0]),
            radius: 100.0,
            material: Box::new(Lambertian::new(
                arr1(&[0.6, 0.6, 0.6, 1.0]),
                Shading::COLOR,
            )),
        }) as Box<dyn RayTraceable>);

        actors.push(Box::new(Sphere {
            center: arr1(&[0.0, 0.0, -1.0, 1.0]),
            radius: 0.5,
            material: Box::new(Lambertian::new(
                arr1(&[0.1, 0.2, 0.5, 1.0]),
                Shading::COLOR,
            )),
        }) as Box<dyn RayTraceable>);

        actors.push(Box::new(Sphere {
            center: arr1(&[1.0, 0.0, -1.0, 1.0]),
            radius: 0.5,
            material: Box::new(Lambertian::new(
                arr1(&[1.0, 0.4, 0.4, 1.0]),
                Shading::COLOR,
            )),
        }) as Box<dyn RayTraceable>);

        actors.push(Box::new(Sphere {
            center: arr1(&[-1.0, 0.0, -1.0, 1.0]),
            radius: 0.5,
            material: Box::new(Lambertian::new(
                arr1(&[0.1, 0.8, 0.2, 1.0]),
                Shading::COLOR,
            )),
        }) as Box<dyn RayTraceable>);

        let dims: [u32; 2] = [200, 100];
        let camera = Camera::new(
            50.0,
            dims[0],
            dims[1],
            arr1(&[-2.0, 2.0, 1.0, 1.0]),
            arr1(&[0.0, 0.0, -1.0, 1.0]),
            arr1(&[0.0, 1.0, 0.0, 0.0]),
        );

        let canvas = Canvas::new(dims[0], dims[1], actors, 5, camera);
        let image = canvas.render_scene();
        let image_png =
            image::RgbaImage::from_raw(dims[0], dims[1], image.data).unwrap();
        let _result = image_png.save(output_path);
        assert_eq!(1.0, 1.0);
    }

    #[test]
    fn camera_fov() {
        let mut output_path = init_image_testing();
        output_path.push("camera_fov.png");

        let rad = std::f64::consts::PI / 4.0;

        let mut actors = vec![];
        actors.push(Box::new(Sphere {
            center: arr1(&[-rad, 0.0, -1.0, 1.0]),
            radius: rad,
            material: Box::new(Lambertian::new(
                arr1(&[0.0, 0.0, 1.0, 1.0]),
                Shading::COLOR,
            )),
        }) as Box<dyn RayTraceable>);

        actors.push(Box::new(Sphere {
            center: arr1(&[rad, 0.0, -1.0, 1.0]),
            radius: rad,
            material: Box::new(Lambertian::new(
                arr1(&[1.0, 0.0, 0.0, 1.0]),
                Shading::COLOR,
            )),
        }) as Box<dyn RayTraceable>);

        let dims: [u32; 2] = [200, 100];
        let camera = Camera::new(
            90.0,
            dims[0],
            dims[1],
            arr1(&[0.0, 0.0, 0.0, 1.0]),
            arr1(&[0.0, 0.0, -1.0, 1.0]),
            arr1(&[0.0, 1.0, 0.0, 0.0]),
        );
        let canvas = Canvas::new(dims[0], dims[1], actors, 1, camera);
        let image = canvas.render_scene();
        let image_png =
            image::RgbaImage::from_raw(dims[0], dims[1], image.data).unwrap();
        let _result = image_png.save(output_path);
        assert_eq!(1.0, 1.0);
    }

    #[test]
    fn render_metal() {
        let mut output_path = init_image_testing();
        output_path.push("render_metal.png");

        let mut actors = vec![];

        actors.push(Box::new(Sphere {
            center: arr1(&[0.0, -100.5, -1.0, 1.0]),
            radius: 100.0,
            material: Box::new(Lambertian::new(
                arr1(&[0.6, 0.6, 0.6, 1.0]),
                Shading::COLOR,
            )),
        }) as Box<dyn RayTraceable>);

        actors.push(Box::new(Sphere {
            center: arr1(&[0.0, 0.0, -1.0, 1.0]),
            radius: 0.5,
            material: Box::new(Lambertian::new(
                arr1(&[0.1, 0.2, 0.5, 1.0]),
                Shading::COLOR,
            )),
        }) as Box<dyn RayTraceable>);

        actors.push(Box::new(Sphere {
            center: arr1(&[1.0, 0.0, -1.0, 1.0]),
            radius: 0.5,
            material: Box::new(Metal::new(
                arr1(&[1.0, 0.4, 0.4, 1.0]),
                Shading::COLOR,
                0.8,
            )),
        }) as Box<dyn RayTraceable>);

        actors.push(Box::new(Sphere {
            center: arr1(&[-1.0, 0.0, -1.0, 1.0]),
            radius: 0.5,
            material: Box::new(Metal::new(
                arr1(&[0.8, 0.8, 0.8, 1.0]),
                Shading::COLOR,
                0.0,
            )),
        }) as Box<dyn RayTraceable>);

        let dims: [u32; 2] = [200, 100];
        let camera = Camera::new(
            90.0,
            dims[0],
            dims[1],
            arr1(&[0.0, 0.0, 0.0, 1.0]),
            arr1(&[0.0, 0.0, -1.0, 1.0]),
            arr1(&[0.0, 1.0, 0.0, 0.0]),
        );

        let canvas = Canvas::new(dims[0], dims[1], actors, 3, camera);
        let image = canvas.render_scene();
        let image_png =
            image::RgbaImage::from_raw(dims[0], dims[1], image.data).unwrap();
        let _result = image_png.save(output_path);
        assert_eq!(1.0, 1.0);
    }

    #[test]
    fn render_dielectric() {
        let mut output_path = init_image_testing();
        output_path.push("render_dielectric.png");

        let mut actors = vec![];

        actors.push(Box::new(Sphere {
            center: arr1(&[0.0, -100.5, -1.0, 1.0]),
            radius: 100.0,
            material: Box::new(Lambertian::new(
                arr1(&[0.8, 0.8, 0.0, 1.0]),
                Shading::COLOR,
            )),
        }) as Box<dyn RayTraceable>);

        actors.push(Box::new(Sphere {
            center: arr1(&[0.0, 0.0, -1.0, 1.0]),
            radius: 0.5,
            material: Box::new(Dielectric::new(
                arr1(&[1.0, 1.0, 1.0, 1.0]),
                Shading::COLOR,
                1.5,
            )),
        }) as Box<dyn RayTraceable>);

        /*
        actors.push(Box::new(Sphere {
            center: arr1(&[0.0, 0.0, -1.0, 1.0]),
            radius: 0.5,
            material: Box::new(Lambertian::new(
                arr1(&[0.1, 0.2, 0.5, 1.0]),
                Shading::COLOR,
            )),
        }) as Box<dyn RayTraceable>);
        */

        /*
        actors.push(Box::new(Sphere {
            center: arr1(&[1.0, 0.0, -1.0, 1.0]),
            radius: 0.5,
            material: Box::new(Metal::new(
                arr1(&[0.8, 0.6, 0.2, 1.0]),
                Shading::COLOR,
                0.0,
            )),
        }) as Box<dyn RayTraceable>);

        */
        actors.push(Box::new(Sphere {
            center: arr1(&[-1.0, 0.0, -1.0, 1.0]),
            radius: 0.5,
            material: Box::new(Dielectric::new(
                arr1(&[1.0, 1.0, 1.0, 1.0]),
                Shading::COLOR,
                1.5,
            )),
        }) as Box<dyn RayTraceable>);

        let dims: [u32; 2] = [200, 100];
        let camera = Camera::new(
            90.0,
            dims[0],
            dims[1],
            arr1(&[0.0, 0.0, 0.0, 1.0]),
            arr1(&[0.0, 0.0, -1.0, 1.0]),
            arr1(&[0.0, 1.0, 0.0, 0.0]),
        );

        let canvas = Canvas::new(dims[0], dims[1], actors, 1, camera);
        let image = canvas.render_scene();
        let image_png =
            image::RgbaImage::from_raw(dims[0], dims[1], image.data).unwrap();
        let _result = image_png.save(output_path);
        assert_eq!(1.0, 1.0);
    }
}
