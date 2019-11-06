use crate::raytracer::actor::Sphere;
use crate::raytracer::actor::Shading;
use crate::raytracer::canvas::Canvas;
use ndarray::arr1;

#[no_mangle]
pub extern "C" fn get_frame() -> *const u8 {
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
    let image_ptr = image.into_raw().as_ptr();

    let value: f64 = 0.5565;
    println!(">>> Rendered frame in {} ms !!", value);

    //let result: f64 = 0.666;
    //result
    image_ptr
}
