use crate::raytracer::actor::Shading;
use crate::raytracer::actor::Sphere;
use crate::raytracer::canvas::Canvas;
use ndarray::arr1;
use std::time::Instant;
use std::mem;

type Frame = image::RgbaImage;


#[no_mangle]
pub extern "C" fn get_frame() -> Box<Frame> {
    // TODO ensure one does not need to create a new
    // canvas every time (single allocation).
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

    //    let image_ptr = image.clone().into_raw().as_ptr();
    //    let ref mut out = File::create(&Path::new("test_rust.png")).unwrap();
    //    let _result = image::ImageRgba8(image).save(out, image::PNG);

    // The Box smart pointer ensures the instance outlives the
    // underlying data pointer.
    Box::new(image)
}

#[no_mangle]
pub extern "C" fn get_width(ptr: *mut Frame) -> u32 {
    let frame = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    frame.width()
}

#[no_mangle]
pub extern "C" fn get_height(ptr: *mut Frame) -> u32 {
    let frame = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    frame.height()
}

#[no_mangle]
pub extern "C" fn get_value(ptr: *mut Frame, x: u32, y: u32, c: u32) -> u8 {
    let frame = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    frame.get_pixel(x, y)[c as usize]
}

#[no_mangle]
pub extern "C" fn get_data(ptr: *mut Frame) -> *const u8 {
    let frame = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    // Found (a similar) solution with `rustc --explain E0507`
    //
    // TODO Need to dobule check if the original content gets replaced
    // back after this (otherwise it might be dropped after this function
    // goes out of scope.
    //
    let some_img = image::RgbaImage::new(10, 10);
    let data_ptr: *const u8 = mem::replace(frame, some_img).into_raw().as_ptr();

    //std::ptr::null()
    data_ptr
}
