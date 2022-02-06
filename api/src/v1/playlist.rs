use crate::common::{platform, util};
use actix_web::{self as web, web::Query, HttpResponse};

#[derive(serde::Deserialize, Debug)]
pub struct PlaylistRequest {
  #[serde(deserialize_with = "platform::known")]
  pub platform: String,
  pub id: String,
  #[serde(default)]
  #[serde(deserialize_with = "util::loose_bool::deserialize")]
  pub shuffle: bool,
  #[serde(default)]
  pub offset: u64,
  #[serde(default)]
  pub limit: u64,
}

#[web::get("/playlist")]
pub async fn get(Query(query): Query<PlaylistRequest>) -> HttpResponse {
  log::info!("{query:#?}");
  HttpResponse::Ok().finish()
}
