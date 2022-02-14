pub mod client;
pub mod common;
#[macro_use]
pub mod db;
pub mod error;
pub mod v1;

use actix_cors::Cors;
use actix_web::{self as web, dev::Server, http::header, middleware, web::Data, App, HttpResponse, HttpServer};
use common::config::Config;
use std::net::TcpListener;

#[web::get("/health")]
async fn health() -> HttpResponse {
  HttpResponse::Ok().finish()
}

pub async fn start(socket: TcpListener, config: Config) -> anyhow::Result<Server> {
  let db = db::connect(&config.database_url).await?;
  let yt = client::Youtube::new("https://www.googleapis.com/youtube/v3", config.youtube_key.clone());
  Ok(
    HttpServer::new(move || {
      App::new()
        .app_data(Data::new(db.clone()))
        .app_data(Data::new(yt.clone()))
        .app_data(Data::new(config.clone()))
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
