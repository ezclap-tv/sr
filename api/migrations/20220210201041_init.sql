CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE TABLE songs (
  song_id           SERIAL PRIMARY KEY,
  added_at          TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  published_at    	TIMESTAMPTZ NOT NULL,
  platform          TEXT NOT NULL,
  platform_song_id  TEXT NOT NULL,
  title             TEXT NOT NULL,
  UNIQUE (platform, platform_song_id)
);

CREATE INDEX index__songs__title__trigram ON songs USING GIN(title gin_trgm_ops);

CREATE TABLE playlists (
  playlist_id          SERIAL PRIMARY KEY,
  updated_at           TIMESTAMPTZ NOT NULL,
  platform             TEXT NOT NULL,
  platform_playlist_id TEXT NOT NULL,
  UNIQUE (platform, platform_playlist_id)
);

CREATE TABLE playlists_songs (
  playlist_id INTEGER REFERENCES playlists(playlist_id),
  song_id     INTEGER REFERENCES songs(song_id),
  PRIMARY KEY (playlist_id, song_id)
);
