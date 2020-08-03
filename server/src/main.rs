#![feature(proc_macro_hygiene, decl_macro)]

extern crate rendering;

#[macro_use]
extern crate rocket;

extern crate image;

use std::path::PathBuf;
use std::path::Path;

use rendering::raytracer::external;
use rendering::raytracer::canvas::Canvas;

use rocket::http::ContentType;
use rocket::response::content::Content;
use rocket::response::NamedFile;
use rocket::response::status::NotFound;

pub struct Renderer {
    canvas: Box<Canvas>
}

pub fn create_renderer(scene_id: u32) -> Renderer {
    let canvas = external::get_renderer(scene_id);
    Renderer {
        canvas,
    }
}

#[get("/api/v1/render")]
fn get_frame() -> Content<Vec<u8>> {
    let renderer = create_renderer(0);
    let image = renderer.canvas.render_scene();

    let mut buffer: Vec<u8> = Vec::new();
    let myimage: Vec<u8> = image.data;
    let imagergba = image::RgbaImage::from_raw(image.width, image.height, myimage.clone());
    let image_png = image::DynamicImage::ImageRgba8(imagergba.unwrap());

    let _result = image_png.write_to(&mut buffer,image::ImageOutputFormat::PNG);

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
    rocket::ignite().mount("/", routes![health, get_frame, get_file]).launch();
}
