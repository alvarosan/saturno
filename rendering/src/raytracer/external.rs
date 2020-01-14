use crate::raytracer::actor::RayTraceable;
use crate::raytracer::actor::Sphere;
use crate::raytracer::camera::Camera;
use crate::raytracer::canvas::Canvas;
use crate::raytracer::material::Lambertian;
use crate::raytracer::material::Shading;
use crate::raytracer::Image;
//extern crate web_sys;

//use web_sys::console;
use ndarray::arr1;
//use std::time::Instant;

pub type Frame = Image;

#[no_mangle]
pub extern "C" fn get_frame() -> Box<Frame> {
    // TODO ensure one does not need to create a new
    // canvas every time (single allocation).
    //println!(">>> Entered get_frame !!");

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
            arr1(&[0.0, 1.0, 0.0, 1.0]),
            Shading::NORMALS,
        )),
    }) as Box<dyn RayTraceable>);

    let from = arr1(&[0.0, 0.0, 0.0, 1.0]);
    let at = arr1(&[0.0, 0.0, -1.0, 1.0]);
    let camera = Camera::new(
        90.0,
        200,
        100,
        from,
        at,
        arr1(&[0.0, 1.0, 0.0, 0.0]),
        2.0,
    );

    let canvas = Canvas::new(200, 100, actors, 10, camera);

    //let now = Instant::now();
    //console::log_1(&"Before canvas::render_scene".into());
    let image = canvas.render_scene();
    //console::log_1(&"After canvas::render_scene".into());
    //println!(">>> Rendered frame in {} ms !!", now.elapsed().as_millis());

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
