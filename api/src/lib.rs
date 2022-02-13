pub mod client;
pub mod common;
pub mod db;
pub mod v1;

use actix_cors::Cors;
use actix_web::{self as web, dev::Server, http::header, middleware, App, HttpResponse, HttpServer};
use std::net::TcpListener;

#[web::get("/health")]
async fn health() -> HttpResponse {
  HttpResponse::Ok().finish()
}

pub fn run(socket: TcpListener) -> std::io::Result<Server> {
  Ok(
    HttpServer::new(move || {
      App::new()
        .wrap(
          Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
            .supports_credentials()
            .max_age(3600),
        )
        .wrap(middleware::Compress::default())
        .wrap(middleware::Logger::default())
        .service(health)
        .service(v1::routes())
    })
    .listen(socket)?
    .run(),
  )
}
