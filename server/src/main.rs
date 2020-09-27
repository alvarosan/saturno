#![feature(proc_macro_hygiene, decl_macro)]

extern crate rendering;

#[macro_use]
extern crate rocket;

extern crate image;

use std::path::Path;
use std::path::PathBuf;

use rendering::raytracer::canvas::Canvas;
use rendering::raytracer::scenes;

use rocket::http::ContentType;
use rocket::response::content::Content;
use rocket::response::status::NotFound;
use rocket::response::NamedFile;

pub struct Renderer {
    canvas: Box<Canvas>,
}

pub fn create_renderer(scene_id: u32) -> Renderer {
    let canvas = scenes::get_renderer(scene_id);
    Renderer { canvas }
}

#[get("/api/v1/render")]
fn get_frame() -> Content<Vec<u8>> {
    let mut renderer = create_renderer(0);
    renderer.canvas.render_scene();
    let image = renderer.canvas.grab_frame();

    let mut buffer: Vec<u8> = Vec::new();

    let buf: Vec<u8> =
        image.data.iter().flat_map(|pixel| pixel.data.iter()).cloned().collect();
    let imagergba =
        image::RgbaImage::from_raw(image.width, image.height, buf.clone());
    let image_png = image::DynamicImage::ImageRgba8(imagergba.unwrap());

    let _result =
        image_png.write_to(&mut buffer, image::ImageOutputFormat::PNG);

    Content(ContentType::PNG, buffer)
}

#[get("/health")]
fn health() -> &'static str {
    "Ok"
}

#[get("/<file..>")]
fn get_file(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("dist/").join(file);

    NamedFile::open(&path).map_err(|e| NotFound(e.to_string()))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![health, get_frame, get_file])
        .launch();
}
