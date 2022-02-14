use crate::{
  client::Youtube,
  common::{config::Config, platform::Platform, util},
  db::{self, playlists::PlaylistData, songs::SongData, Database},
  error::FailWith,
};
use actix_web::{get, web, web::Query, HttpResponse, Result};
use chrono::{Duration, Utc};

fn default_limit() -> u64 {
  10
}

#[derive(serde::Deserialize, Debug)]
pub struct PlaylistRequest {
  pub platform: Platform,
  pub id: String,
  // TODO: shuffle
  /* #[serde(default)]
  #[serde(deserialize_with = "util::loose_bool::deserialize")]
  pub shuffle: bool, */
  #[serde(default)]
  pub offset: u64,
  #[serde(default = "default_limit")]
  pub limit: u64,
  #[serde(default)]
  #[serde(deserialize_with = "util::loose_bool::deserialize")]
  pub force: bool,
}

async fn should_fetch(db: &Database, platform: Platform, id: &str, refresh_interval: Duration) -> Result<bool> {
  Ok(match db::playlists::get(db, platform, id).await.internal()? {
    Some(playlist) => Utc::now() > *playlist.updated_at() + refresh_interval,
    None => true,
  })
}

#[get("/playlist")]
pub async fn get(
  config: web::Data<Config>,
  db: web::Data<Database>,
  client: web::Data<Youtube>,
  Query(query): Query<PlaylistRequest>,
) -> Result<HttpResponse> {
  log::info!("{query:#?}");
  // check if playlist exists + get last updated time
  // update and persist can probably be the same (just INSERT INTO ... ON CONFLICT DO NOTHING), but `update` also has to modify `updated_at`
  // if should update: fetch + update playlist
  // if does not exist: fetch + persist playlist
  if should_fetch(
    db.get_ref(),
    query.platform,
    &query.id,
    config.playlist_refresh_interval,
  )
  .await?
  {
    let data = match query.platform {
      Platform::Youtube => PlaylistData::new(
        Platform::Youtube,
        query.id.clone(),
        client
          .get_ref()
          .playlist_videos(&query.id)
          .await
          .with("Failed to fetch playlist from YouTube")?
          .into_iter()
          .map(SongData::from)
          .collect::<Vec<_>>(),
      ),
    };
    db::playlists::upsert(db.get_ref(), data).await.internal()?;
  }

  // return playlist page(offset, limit)
  Ok(
    HttpResponse::Ok().json(
      db::playlists::get_page(db.get_ref(), &query.id, query.offset as i32, query.limit as i32)
        .await
        .internal()?,
    ),
  )
}
