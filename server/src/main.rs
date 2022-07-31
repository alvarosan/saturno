extern crate image;
extern crate rendering;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;

pub mod renderer;
use crate::renderer::render_frame;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .init();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET"])
            .allowed_headers(vec!["*"])
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .service(health)
            .service(render_frame)
    })
    .bind(("0.0.0.0", 8082))?
    .run()
    .await
}

///////////////////////////////////////////////////////////////////////////////
// Services
///////////////////////////////////////////////////////////////////////////////

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}
