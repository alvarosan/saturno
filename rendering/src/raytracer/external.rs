//use crate::raytracer::actor::RayTraceable;
//use crate::raytracer::camera::Camera;
//use crate::raytracer::canvas::Canvas;
//use crate::raytracer::scenes;
//use crate::raytracer::image::Image;
//use ndarray::arr1;
//
////extern crate web_sys;
////use web_sys::console;
////use std::time::Instant;
//
//pub type Frame = Image;
//
//#[no_mangle]
//pub extern "C" fn get_renderer(scene_id: u32) -> Box<Canvas> {
//    let dims: [u32; 2] = [200, 133];
//    let actors: Vec<Box<dyn RayTraceable>>;
//    let camera: Camera;
//    match scene_id {
//        0 => {
//            actors = scenes::random_book_cover();
//            camera = Camera::new(
//                20.0,
//                dims[0],
//                dims[1],
//                arr1(&[13.0, 2.0, 3.0, 1.0]),
//                arr1(&[0.0, 0.0, 0.0, 1.0]),
//                arr1(&[0.0, 1.0, 0.0, 0.0]),
//                0.2,
//            );
//        }
//        _ => {
//            actors = scenes::two_spheres_normals();
//            camera = Camera::new(
//                90.0,
//                dims[0],
//                dims[1],
//                arr1(&[0.0, 0.0, 0.0, 1.0]),
//                arr1(&[0.0, 0.0, -1.0, 1.0]),
//                arr1(&[0.0, 1.0, 0.0, 0.0]),
//                0.0,
//            );
//        }
//    }
//
//    Box::new(Canvas::new(dims[0], dims[1], actors, 2, camera))
//}
//
//#[no_mangle]
//pub extern "C" fn render_scene(ptr: *mut Canvas) -> Box<Frame> {
//    let canvas = unsafe {
//        assert!(!ptr.is_null());
//        &mut *ptr
//    };
//
//    canvas.render_scene();
//    Box::new(canvas.grab_frame())
//}
//
// /**
// * This function recreates the entire renderer on every
// * frame.
// */
//#[no_mangle]
//pub extern "C" fn get_frame() -> Box<Frame> {
//    let dims: [u32; 2] = [200, 133];
//    let actors = scenes::random_book_cover();
//    let camera = Camera::new(
//        20.0,
//        dims[0],
//        dims[1],
//        arr1(&[13.0, 2.0, 3.0, 1.0]),
//        arr1(&[0.0, 0.0, 0.0, 1.0]),
//        arr1(&[0.0, 1.0, 0.0, 0.0]),
//        0.2,
//    );
//
//    let mut canvas = Canvas::new(200, 100, actors, 2, camera);
//
//    //let now = Instant::now();
//    //console::log_1(&"Before canvas::render_scene".into());
//    canvas.render_scene();
//    let image = canvas.grab_frame();
//    //console::log_1(&"After canvas::render_scene".into());
//
//    // The Box smart pointer ensures the instance outlives the
//    // underlying data pointer.
//    Box::new(image)
//}
//
//#[no_mangle]
//pub extern "C" fn get_width(ptr: *mut Frame) -> u32 {
//    let frame = unsafe {
//        assert!(!ptr.is_null());
//        &mut *ptr
//    };
//    frame.width
//}
//
//#[no_mangle]
//pub extern "C" fn get_height(ptr: *mut Frame) -> u32 {
//    let frame = unsafe {
//        assert!(!ptr.is_null());
//        &mut *ptr
//    };
//    frame.height
//}
//
//#[no_mangle]
//pub extern "C" fn get_value(ptr: *mut Frame, x: u32, y: u32, c: u32) -> u8 {
//    let frame = unsafe {
//        assert!(!ptr.is_null());
//        &mut *ptr
//    };
//    frame.get_value(x, y, c)
//}
//
//#[no_mangle]
//pub extern "C" fn get_data(ptr: *mut Frame) -> *const u8 {
//    let frame = unsafe {
//        assert!(!ptr.is_null());
//        &mut *ptr
//    };
//
//    let data_ptr: *const u8 = frame.data.as_ptr() as *const u8;
//    data_ptr
//}
