use crate::common::platform;
use actix_web::{self as web, web::Query, HttpResponse};

#[derive(serde::Deserialize, Debug)]
pub struct SearchRequest {
  #[serde(deserialize_with = "platform::known_opt")]
  pub platform: Option<String>,
  pub query: String,
}

#[web::get("/search")]
pub async fn get(Query(query): Query<SearchRequest>) -> HttpResponse {
  log::info!("{query:#?}");
  HttpResponse::Ok().finish()
}
