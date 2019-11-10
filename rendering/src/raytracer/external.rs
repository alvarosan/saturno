use crate::raytracer::actor::Shading;
use crate::raytracer::actor::Sphere;
use crate::raytracer::canvas::Canvas;
use ndarray::arr1;
//use std::fs::File;
//use std::path::Path;
use std::time::Instant;

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

    let now = Instant::now();
    let image = canvas.render_scene();
    println!(">>> Rendered frame in {} ms !!", now.elapsed().as_millis());

    // TODO image gets deallocated when this function runs out of scope,
    // therefore it is necessary to move the entire image object or ensure
    // it outlives the pointer. 
    // https://docs.rs/image/0.19.0/image/struct.ImageBuffer.html#method.as_ptr
    //
    // This currently causes a garbagy image in Go.
    let image_ptr = image.into_raw().as_ptr();

//    let image_ptr = image.clone().into_raw().as_ptr();
//    let ref mut out = File::create(&Path::new("test_rust.png")).unwrap();
//    let _result = image::ImageRgba8(image).save(out, image::PNG);

    image_ptr
}
