use super::songs::*;

#[derive(Debug, Clone)]
pub struct PlaylistData {
  platform: String,
  playlist_id: String,
  songs: Vec<SongData>,
}

pub async fn insert<'db, E>(db: E, playlist: PlaylistData) -> sqlx::Result<()>
where
  E: sqlx::PgExecutor<'db> + 'db,
{
  let songs = SongData::soa(playlist.songs);

  // $1 = playlist.platform
  // $2 = playlist.platform_playlist_id
  // $3 = songs.published_at
  // $4 = songs.platform
  // $5 = songs.platform_song_id
  // $6 = songs.title
  sqlx::query(
    r#"
      WITH
      -- insert playlist, get its id
      new_playlist AS (
        INSERT INTO playlists (updated_at, platform, platform_playlist_id)
        VALUES (now()::timestamptz, $1::text, $2::text)
        RETURNING playlist_id
      ),
      -- either select or insert every song, getting all ids
      selected_songs AS (
        SELECT song_id FROM songs
        WHERE (platform, platform_song_id) IN (SELECT * FROM UNNEST($4::text[], $5::text[]))
      ),
      inserted_songs as (
        INSERT INTO songs (published_at, platform, platform_song_id, title)
        SELECT * FROM UNNEST($3::timestamptz[], $4::text[], $5::text[], $6::text[])
        ON CONFLICT DO NOTHING
        RETURNING song_id
      ),
      -- join the selected with the inserted ids
      song_ids AS (
        SELECT * FROM selected_songs
        UNION
        SELECT * FROM inserted_songs
      )
      -- insert into playlists<->songs table
      -- by joining new playlist id with every id from inserted+selected songs
      INSERT INTO playlists_songs (playlist_id, song_id)
      SELECT * FROM new_playlist
      JOIN song_ids ON true
    "#,
  )
  .bind(&playlist.platform)
  .bind(&playlist.playlist_id)
  .bind(&songs.published_at)
  .bind(&songs.platform)
  .bind(&songs.song_id)
  .bind(&songs.title)
  .execute(db)
  .await?;
  Ok(())
}

// get playlist items (keyset pagination)
pub async fn get_page<'db, E>(db: E, playlist_id: &str, offset: i32, limit: i32) -> sqlx::Result<Vec<Song>>
where
  E: sqlx::PgExecutor<'db> + 'db,
{
  sqlx::query_as(
    r#"
      WITH song_ids AS (
        SELECT song_id FROM playlists_songs
        WHERE playlist_id = (
          SELECT playlist_id FROM playlists
          WHERE platform_playlist_id = $1
        )
        OFFSET $2
        LIMIT $3
      )
      SELECT * FROM songs
      WHERE song_id IN (SELECT * FROM song_ids)
    "#,
  )
  .bind(playlist_id)
  .bind(offset)
  .bind(limit)
  .fetch_all(db)
  .await
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::db;
  use chrono::{Duration, Utc};
  use rand::{thread_rng, Rng};
  use uuid::Uuid;

  fn generate_song() -> SongData {
    SongData {
      published_at: Utc::now() - Duration::seconds(thread_rng().gen_range(0..1209600)),
      song_id: Uuid::new_v4().to_string(),
      platform: "test".into(),
      title: "song".into(),
    }
  }

  #[actix_rt::test]
  #[cfg_attr(not(feature = "test-database"), ignore)]
  async fn insert_a_playlist() -> anyhow::Result<()> {
    let conn = db::connect_from_env().await?;
    let mut tx = conn.begin().await?;

    let songs = (0..500).into_iter().map(|_| generate_song()).collect::<Vec<_>>();

    // pre-insert 250 of the 500 generated songs
    db::songs::create_bulk(&mut tx, songs[..250].to_vec()).await?;

    // insert the playlist
    // this should create:
    // - the playlist
    // - the remaining 250 songs
    // - entries in `playlists_songs` table
    let data = PlaylistData {
      platform: "test".into(),
      playlist_id: "test-playlist".into(),
      songs: songs.clone(),
    };
    insert(&mut tx, data.clone()).await?;

    // playlist exists
    assert!(
      sqlx::query_scalar::<_, bool>(r#"SELECT true FROM playlists WHERE platform_playlist_id = $1"#,)
        .bind(&data.playlist_id)
        .fetch_one(&mut tx)
        .await?
    );
    // there are exactly 500 songs
    let song_count = sqlx::query_scalar::<_, i64>(r#"SELECT COUNT(*) FROM songs"#)
      .fetch_one(&mut tx)
      .await?;
    assert_eq!(song_count, 500);
    // all the songs are part of the new playlist
    let playlist_song_count = sqlx::query_scalar::<_, i64>(
      r#"
        SELECT COUNT(*) FROM playlists_songs
        WHERE playlist_id = (
          SELECT playlist_id FROM playlists
          WHERE platform_playlist_id = $1
        )
      "#,
    )
    .bind(&data.playlist_id)
    .fetch_one(&mut tx)
    .await?;
    assert_eq!(song_count, playlist_song_count);

    tx.rollback().await?;
    Ok(())
  }

  #[actix_rt::test]
  #[cfg_attr(not(feature = "test-database"), ignore)]
  async fn paged_playlist() -> anyhow::Result<()> {
    let conn = db::connect_from_env().await?;
    let mut tx = conn.begin().await?;

    // create a playlist
    sqlx::query("INSERT INTO playlists (updated_at, platform, platform_playlist_id) VALUES (now(), 'youtube', 'test')")
      .execute(&mut tx)
      .await?;
    // insert some songs
    let songs = (0..100).into_iter().map(|_| generate_song()).collect::<Vec<_>>();
    db::songs::create_bulk(&mut tx, songs.clone()).await?;
    // insert the playlist entries
    sqlx::query(
      "
      WITH
      playlist AS (SELECT playlist_id FROM playlists WHERE platform_playlist_id = 'test'),
      song_ids AS (SELECT song_id FROM songs)
      INSERT INTO playlists_songs (playlist_id, song_id)
      SELECT * FROM playlist
      JOIN song_ids ON true
    ",
    )
    .execute(&mut tx)
    .await?;

    assert_eq!(get_page(&mut tx, "test", 0, 50).await?.len(), 50);
    assert_eq!(get_page(&mut tx, "test", 50, 50).await?.len(), 50);
    assert_eq!(get_page(&mut tx, "test", 100, 50).await?.len(), 0);

    tx.rollback().await?;
    Ok(())
  }
}
