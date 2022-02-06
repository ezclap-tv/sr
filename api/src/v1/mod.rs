pub mod memo;
pub mod playlist;
pub mod random;
pub mod search;

use actix_web::{web, Scope};

pub fn routes() -> Scope {
  web::scope("/v1")
    .service(memo::post)
    .service(playlist::get)
    .service(random::get)
    .service(search::get)
}
