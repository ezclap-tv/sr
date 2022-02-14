use crate::{
  common::platform::Platform,
  db::{self, Database},
};
use actix_web::{get, web, web::Query, HttpResponse};

#[derive(serde::Deserialize, Debug)]
pub struct SearchRequest {
  pub platform: Option<Platform>,
  pub query: String,
}

#[get("/search")]
pub async fn get(db: web::Data<Database>, Query(query): Query<SearchRequest>) -> HttpResponse {
  log::info!("{query:#?}");
  // TODO: call youtube search api in a reasonable way

  // % => \%
  // _ => \_
  // \ => \\
  // %<INPUT>%

  //db::songs::search_by_title(db.get_ref(), query.)
  HttpResponse::Ok().finish()
}
