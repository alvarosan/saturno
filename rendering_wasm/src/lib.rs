extern crate rendering;

//use rendering::raytracer::external::{get_frame, Frame};
use wasm_bindgen::prelude::*;

mod utils;

//// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
//// allocator.
//#[cfg(feature = "wee_alloc")]
//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, {{project-name}}!");
}

//#[wasm_bindgen]
//pub fn render() -> Box<Frame> {
//    get_frame()
//}
