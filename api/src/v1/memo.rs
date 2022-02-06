use crate::common::platform;
use actix_web::{self as web, web::Json, HttpResponse, Result};

#[derive(serde::Deserialize, Debug)]
pub struct MemoRequest {
  #[serde(deserialize_with = "platform::known")]
  /// Platform identifier, youtube/spotify/soundcloud/etc
  pub platform: String,
  pub id: String,
  pub channel: String,
}

/// Memorize the song, allowing it to be returned from `/random`.
#[web::post("/memo")]
pub async fn post(Json(body): Json<MemoRequest>) -> Result<HttpResponse> {
  println!("{body:#?}");
  Ok(HttpResponse::Ok().finish())
}
