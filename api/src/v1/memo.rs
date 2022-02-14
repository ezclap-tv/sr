use crate::client::Youtube;
use crate::common::platform::Platform;
use crate::db::{songs, Database};
use crate::error::FailWith;
use actix_web::{post, web, web::Json, HttpResponse, Result};

#[derive(serde::Deserialize, Debug)]
pub struct MemoRequest {
  pub platform: Platform,
  pub id: String,
}

/// Memorize the song, allowing it to be returned from `/random`.
#[post("/memo")]
pub async fn post(
  db: web::Data<Database>,
  client: web::Data<Youtube>,
  Json(body): Json<MemoRequest>,
) -> Result<HttpResponse> {
  log::info!("{body:#?}");
  // check if we know this (platform, song_id) combination
  if !songs::exists(db.get_ref(), body.platform, body.id.clone())
    .await
    .internal()?
  {
    // if false: fetch info from youtube/videos
    let (song_id, title, published_at) = match body.platform {
      Platform::Youtube => {
        log::info!("getting video {}", body.id);
        let result = client.videos([body.id.as_str()]).await.with("Invalid song id")?;
        log::info!("{result:#?}");
        let video = result.into_iter().next().with("Invalid song id")?;
        (video.id, video.title, video.published_at)
      }
    };
    // and store it
    log::info!("storing {song_id}, {title}, {published_at}");
    songs::create(
      db.get_ref(),
      songs::SongData::new(published_at, song_id, body.platform, title),
    )
    .await
    .internal()?;
  }
  Ok(HttpResponse::Ok().finish())
}
