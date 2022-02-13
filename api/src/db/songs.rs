use chrono::{DateTime, Utc};

#[derive(Debug, Clone, getset::Getters, sqlx::FromRow)]
#[getset(get = "pub")]
pub struct Song {
  #[sqlx(rename = "song_id")]
  id: i32,
  added_at: DateTime<Utc>,
  published_at: DateTime<Utc>,
  platform: String,
  #[sqlx(rename = "platform_song_id")]
  song_id: String,
  title: String,
}

#[derive(Debug, Clone)]
pub struct SongData {
  pub published_at: DateTime<Utc>,
  pub platform: String,
  pub song_id: String,
  pub title: String,
}

pub struct SongDataSoa {
  pub published_at: Vec<DateTime<Utc>>,
  pub song_id: Vec<String>,
  pub platform: Vec<String>,
  pub title: Vec<String>,
}

impl SongData {
  pub fn new(platform: String, song_id: String, title: String, published_at: DateTime<Utc>) -> Self {
    Self {
      published_at,
      song_id,
      platform,
      title,
    }
  }

  pub fn soa(data: Vec<SongData>) -> SongDataSoa {
    let mut published_at = Vec::with_capacity(data.len());
    let mut song_id = Vec::with_capacity(data.len());
    let mut platform = Vec::with_capacity(data.len());
    let mut title = Vec::with_capacity(data.len());
    for item in data.into_iter() {
      published_at.push(item.published_at);
      song_id.push(item.song_id);
      platform.push(item.platform);
      title.push(item.title);
    }
    SongDataSoa {
      published_at,
      song_id,
      platform,
      title,
    }
  }
}

pub async fn create<'db, E>(db: E, data: SongData) -> sqlx::Result<Song>
where
  E: sqlx::PgExecutor<'db> + 'db,
{
  sqlx::query_as(
    r#"
      WITH
      selected AS (
        SELECT * FROM songs
        WHERE (platform, platform_song_id) = ($2, $3)
      ),
      inserted AS (
        INSERT INTO songs (published_at, platform, platform_song_id, title)
          VALUES ($1, $2, $3, $4)
        ON CONFLICT DO NOTHING
        RETURNING *
      )
      SELECT * FROM selected
      UNION ALL
      SELECT * FROM inserted;
    "#,
  )
  .bind(&data.published_at)
  .bind(&data.platform)
  .bind(&data.song_id)
  .bind(&data.title)
  .fetch_one(db)
  .await
}

pub async fn create_bulk<'db, E>(db: E, data: Vec<SongData>) -> sqlx::Result<()>
where
  E: sqlx::PgExecutor<'db> + 'db,
{
  let songs = SongData::soa(data);
  sqlx::query(
    r#"
      INSERT INTO songs (published_at, platform, platform_song_id, title)
        SELECT * FROM UNNEST($1::timestamptz[], $2::text[], $3::text[], $4::text[])
      ON CONFLICT DO NOTHING;
    "#,
  )
  .bind(&songs.published_at)
  .bind(&songs.platform)
  .bind(&songs.song_id)
  .bind(&songs.title)
  .execute(db)
  .await?;
  Ok(())
}

pub async fn random<'db, E>(db: E) -> sqlx::Result<Song>
where
  E: sqlx::PgExecutor<'db> + 'db,
{
  sqlx::query_as(
    r#"
      SELECT * FROM songs
      OFFSET floor(random() * (SELECT COUNT(*) FROM songs))
      LIMIT 1
    "#,
  )
  .fetch_one(db)
  .await
}

pub async fn find<'db, E>(db: E, title: String) -> sqlx::Result<Song>
where
  E: sqlx::PgExecutor<'db> + 'db,
{
  sqlx::query_as(
    r#"
      SELECT * FROM songs
      WHERE title LIKE $1
    "#,
  )
  .bind(&title)
  .fetch_one(db)
  .await
}

#[cfg(test)]
mod tests {
  use super::*;
}
