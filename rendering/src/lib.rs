pub mod mandelbrot;
pub mod raytracer;

#[cfg(test)]
mod tests {
    use ndarray::arr1;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn point_at_parameter() {
        let ray = crate::raytracer::ray::Ray {
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
        let canvas = crate::raytracer::canvas::Canvas {
            width: 200,
            height: 100,
        };

        let image = canvas.render_background();
        let ref mut out = File::create(&Path::new("background.png")).unwrap();
        let _result = image::ImageRgba8(image).save(out, image::PNG);
        assert_eq!(1.0, 1.0);
    }
}
