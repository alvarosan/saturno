use crate::raytracer::actor::Shading;
use crate::raytracer::actor::Sphere;
use crate::raytracer::canvas::Canvas;
use crate::raytracer::Image;
use ndarray::arr1;
use std::time::Instant;

pub type Frame = Image;

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
    frame.width
}

#[no_mangle]
pub extern "C" fn get_height(ptr: *mut Frame) -> u32 {
    let frame = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    frame.height
}

#[no_mangle]
pub extern "C" fn get_value(ptr: *mut Frame, x: u32, y: u32, c: u32) -> u8 {
    let frame = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    frame.get_value(x, y, c)
}

#[no_mangle]
pub extern "C" fn get_data(ptr: *mut Frame) -> *const u8 {
    let frame = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let data_ptr: *const u8 = frame.data.as_ptr();
    data_ptr
}
