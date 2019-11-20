extern crate rendering;

use rendering::raytracer::external::get_frame;
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

#[wasm_bindgen]
pub fn render() -> ByteStream {
    let frame = get_frame();
    ByteStream::new(&frame.data, frame.width, frame.height)
}

#[wasm_bindgen]
pub struct ByteStream {
    data: *const u8,
    width: u32,
    height: u32,
    size: usize,
}

#[wasm_bindgen]
impl ByteStream {
    pub fn new(bytes: &[u8], width: u32, height: u32) -> ByteStream {
        ByteStream {
            data: bytes.as_ptr(),
            size: bytes.len(),
            width,
            height,
        }
    }

    pub fn data(&self) -> *const u8 {
        self.data
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
