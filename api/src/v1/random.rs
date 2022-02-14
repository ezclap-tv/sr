use crate::{
  common::platform::Platform,
  db::{self, Database},
  error::FailWith,
};
use actix_web::{get, web, web::Query, HttpResponse, Result};

#[derive(serde::Deserialize, Debug)]
pub struct RandomRequest {
  pub platform: Option<Platform>,
  pub channel: Option<String>,
  #[serde(default = "default_count")]
  pub count: u64,
}

fn default_count() -> u64 {
  1
}

#[get("/random")]
pub async fn get(db: web::Data<Database>, Query(query): Query<RandomRequest>) -> Result<HttpResponse> {
  log::info!("{query:#?}");
  // fetch and return a random song id
  Ok(HttpResponse::Ok().json(db::songs::random(db.get_ref()).await.internal()?))
}
