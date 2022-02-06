pub mod common;
pub mod v1;

use std::net::SocketAddr;

use actix_cors::Cors;
use actix_web::{http::header, middleware, App, HttpServer};
use anyhow::Result;

pub async fn run(addr: SocketAddr) -> Result<()> {
  let server = HttpServer::new(move || {
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
      .service(v1::routes())
  });

  Ok(server.bind(addr)?.run().await?)
}
