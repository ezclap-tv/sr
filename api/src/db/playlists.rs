use super::{songs::*, Database};
use crate::common::platform::Platform;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, sqlx::FromRow, getset::Getters)]
#[getset(get = "pub")]
pub struct Playlist {
  #[sqlx(rename = "playlist_id")]
  id: i32,
  updated_at: DateTime<Utc>,
  platform: Platform,
  #[sqlx(rename = "platform_playlist_id")]
  playlist_id: String,
}

#[derive(Debug, Clone)]
pub struct PlaylistData {
  platform: Platform,
  playlist_id: String,
  songs: Vec<SongData>,
}

impl PlaylistData {
  pub fn new(platform: Platform, id: String, songs: Vec<SongData>) -> Self {
    Self {
      platform,
      playlist_id: id,
      songs,
    }
  }
}

pub async fn get(db: &Database, platform: Platform, id: &str) -> sqlx::Result<Option<Playlist>> {
  sqlx::query_as(r#"SELECT * FROM playlists WHERE (platform, platform_playlist_id) = ($1, $2)"#)
    .bind(platform.as_str())
    .bind(id)
    .fetch_optional(db)
    .await
}

/// Insert or update a playlist
///
/// - Creates a `playlists` table entry, or sets its `updated_at = now()` on conflict
/// - Inserts new songs from `PlaylistData.songs`
/// - Deletes all `playlists_songs` entries with `playlist_id = playlist.id`
/// - Creates `playlists_songs` entries, joining `playlist.id` with every `id` in new songs
///
pub async fn upsert(db: &Database, playlist: PlaylistData) -> sqlx::Result<()> {
  let mut tx = db.begin().await?;

  let playlist_id: i32 = sqlx::query_scalar(
    r#"
      INSERT INTO playlists (updated_at, platform, platform_playlist_id)
      VALUES (now()::timestamptz, $1::text, $2::text)
      ON CONFLICT (platform, platform_playlist_id) DO UPDATE SET updated_at = now()
      RETURNING playlist_id
    "#,
  )
  .bind(playlist.platform.as_str())
  .bind(&playlist.playlist_id)
  .fetch_one(&mut tx)
  .await?;

  sqlx::query(
    r#"
      DELETE FROM playlists_songs
      WHERE playlist_id = $1
    "#,
  )
  .bind(playlist_id)
  .execute(&mut tx)
  .await?;

  let songs = SongData::soa(playlist.songs);
  // TODO: order by song position when inserting
  sqlx::query(
    r#"
      WITH
      -- either select or insert every song, getting all ids
      selected_songs AS (
        SELECT song_id FROM songs
        WHERE (platform, platform_song_id) IN (SELECT * FROM UNNEST($3::text[], $4::text[]))
      ),
      inserted_songs as (
        INSERT INTO songs (published_at, platform, platform_song_id, title)
        SELECT * FROM UNNEST($2::timestamptz[], $3::text[], $4::text[], $5::text[])
        ON CONFLICT DO NOTHING
        RETURNING song_id
      ),
      -- join the selected with the inserted ids
      song_ids AS (
        SELECT * FROM selected_songs
        UNION
        SELECT * FROM inserted_songs
        ORDER BY song_id
      )
      -- insert into playlists<->songs table
      INSERT INTO playlists_songs (playlist_id, song_id)
      SELECT $1 AS playlist_id, song_id FROM song_ids
      ON CONFLICT DO NOTHING
    "#,
  )
  .bind(&playlist_id)
  .bind(&songs.published_at)
  .bind(&songs.platform)
  .bind(&songs.song_id)
  .bind(&songs.title)
  .execute(&mut tx)
  .await?;

  tx.commit().await?;
  Ok(())
}

// get playlist items (keyset pagination)
pub async fn get_page(db: &Database, playlist_id: &str, offset: i32, limit: i32) -> sqlx::Result<Vec<Song>> {
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

// TODO: start a new logical database for every test run to allow for using transactions in queries
/* #[cfg(test)]
mod tests {
  use super::*;
  use crate::{common::platform::Platform, db};
  use chrono::{Duration, Utc};
  use rand::{thread_rng, Rng};
  use uuid::Uuid;

  fn generate_song() -> SongData {
    SongData::new(
      Utc::now() - Duration::seconds(thread_rng().gen_range(0..1209600)),
      Uuid::new_v4().to_string(),
      Platform::Youtube,
      "song".into(),
    )
  }

  crate::db_test!(retrieve_playlist, tx {
    sqlx::query(
      r#"
        INSERT INTO playlists (updated_at, platform, platform_playlist_id)
        VALUES (now(), 'youtube', 'test-id')
      "#,
    )
    .execute(&mut tx)
    .await?;

    let playlist = get(&mut tx, Platform::Youtube, "test-id").await?;
    assert!(playlist.is_some());
    let playlist = playlist.unwrap();
    assert_eq!(playlist.platform, Platform::Youtube);
    assert_eq!(&playlist.playlist_id, "test-id");
  });

  crate::db_test!(insert_and_update_playlist, tx {
    let songs = (0..500).into_iter().map(|_| generate_song()).collect::<Vec<_>>();
    // stage 1: insert a new playlist (with new songs)
    {
      // pre-insert 250 of the 500 generated songs
      db::songs::create_bulk(&mut tx, songs[..250].to_vec()).await?;

      // insert the playlist
      // this should create:
      // - the playlist
      // - the remaining 250 songs
      // - entries in `playlists_songs` table
      let data = PlaylistData {
        platform: Platform::Youtube,
        playlist_id: "test-playlist".into(),
        songs: songs.clone(),
      };
      upsert(&mut tx, data.clone()).await?;

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
    }

    let songs = songs
      .into_iter()
      .chain((0..100).into_iter().map(|_| generate_song()))
      .collect::<Vec<_>>();
    // stage 2: update the playlist, with some songs being unique, others not
    {
      let data = PlaylistData {
        platform: Platform::Youtube,
        playlist_id: "test-playlist".into(),
        songs: songs.clone(),
      };
      upsert(&mut tx, data.clone()).await?;

      // there are exactly 600 songs
      let song_count = sqlx::query_scalar::<_, i64>(r#"SELECT COUNT(*) FROM songs"#)
        .fetch_one(&mut tx)
        .await?;
      assert_eq!(song_count, 600);

      // all the songs are part of the playlist
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
    }
  });

  crate::db_test!(paged_playlist, tx {
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
  });
} */
