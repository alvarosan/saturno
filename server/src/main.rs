#![feature(proc_macro_hygiene, decl_macro)]

extern crate rendering;

#[macro_use]
extern crate rocket;

use rendering::raytracer::external;
use rendering::raytracer::canvas::Canvas;

// TODO
//#[get("/api/v1/render")]

pub struct Renderer {
    canvas: Box<Canvas>
}

pub fn create_renderer(scene_id: u32) -> Renderer {
    let canvas = external::get_renderer(scene_id);
    Renderer {
        canvas,
    }
}

#[get("/")]
fn index() -> &'static str {
    let renderer = create_renderer(0);
    "Created renderer!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
