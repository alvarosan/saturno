use actix_web::{get, HttpResponse, Responder};

use rendering::raytracer::canvas::Canvas;
use rendering::raytracer::scenes;

pub struct Renderer {
    canvas: Box<Canvas>,
}

pub fn create_renderer(scene_id: u32) -> Renderer {
    let canvas = scenes::get_renderer(scene_id);
    Renderer { canvas }
}

#[get("/api/v1/render")]
async fn render_frame() -> impl Responder {
    let mut renderer = create_renderer(0);
    renderer.canvas.render_scene();
    let image = renderer.canvas.grab_frame();

    let mut buffer: Vec<u8> = Vec::new();

    let buf: Vec<u8> = image
        .data
        .iter()
        .flat_map(|pixel| pixel.data.iter())
        .cloned()
        .collect();
    let imagergba = image::RgbaImage::from_raw(image.width, image.height, buf);
    let image_png = image::DynamicImage::ImageRgba8(imagergba.unwrap());

    let _result =
        image_png.write_to(&mut buffer, image::ImageOutputFormat::PNG);

    HttpResponse::Ok().content_type("image/png").body(buffer)
}
