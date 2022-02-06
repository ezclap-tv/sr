use actix_web::{self as web, web::Query, HttpResponse};
use crate::common::platform;

#[derive(serde::Deserialize, Debug)]
pub struct RandomRequest {
  #[serde(deserialize_with = "platform::known")]
  pub platform: String,
  pub channel: Option<String>,
  #[serde(default = "default_count")]
  pub count: u64,
}

fn default_count() -> u64 {
  1
}

#[web::get("/random")]
pub async fn get(Query(query): Query<RandomRequest>) -> HttpResponse {
  log::info!("{query:#?}");
  HttpResponse::Ok().finish()
}
