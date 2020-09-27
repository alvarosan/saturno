extern crate rendering;

use rendering::raytracer::canvas::Canvas;
use rendering::raytracer::scenes;
use wasm_bindgen::prelude::*;
use web_sys::console;

mod utils;

use utils::set_panic_hook;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("He, {{project-name}}! llo");
}

#[wasm_bindgen]
pub fn create_renderer(scene_id: u32) -> Renderer {
    let canvas = scenes::get_renderer(scene_id);
    Renderer { canvas }
}

///////////////////////////////////////////////////////////////////////////////
// Wasm wrappers
///////////////////////////////////////////////////////////////////////////////
#[wasm_bindgen]
pub struct Renderer {
    canvas: Box<Canvas>,
}

#[wasm_bindgen]
impl Renderer {
    pub fn render(&mut self) -> ByteStream {
        set_panic_hook();

        console::log_1(&"Calling render_scene()".into());
        self.canvas.render_scene();
        let frame = self.canvas.grab_frame();

        let buf: Vec<u8> = frame
            .data
            .iter()
            .flat_map(|pixel| pixel.data.iter())
            .cloned()
            .collect();
        ByteStream::new(&buf, frame.width, frame.height)
    }
}

#[wasm_bindgen]
pub struct ByteStream {
    data: *const u8,
    width: u32,
    height: u32,
    size: usize,
    byte_size: usize,
}

#[wasm_bindgen]
impl ByteStream {
    pub fn new(bytes: &[u8], width: u32, height: u32) -> ByteStream {
        let byte_size = width as usize * height as usize * 4 as usize;
        ByteStream {
            byte_size: byte_size,
            data: bytes.as_ptr(),
            size: bytes.len(),
            width,
            height,
        }
    }

    pub fn data(&self) -> *const u8 {
        self.data
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn byte_size(&self) -> usize {
        self.byte_size
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
