### GET /playlist

  ?platform=PLATFORM - (required) Platform identifier, youtube/spotify/soundcloud/etc
  &id=ID             - (required) Playlist ID
  &shuffle=SHUFFLE   - (optional) Songs will be returned in a random order
  &channel=CHANNEL   - (optional) Channel name of the chat that requested this song
  &offset=OFFSET     - (optional) Pagination offset, ignored if `shuffle` is true, default 0
  &limit=LIMIT       - (optional) Pagination limit, default 10

Obtain a list of songs from a playlist on a given platform.
Shuffling works by retrieving the entire playlist at once, and randomly selecting N=limit songs.

### GET /random

  ?platform=PLATFORM - (optional) Platform identifier, youtube/spotify/soundcloud/etc
  &count=COUNT       - (optional) Number of random songs to return, default 1

Obtain N=count random songs (any platform)

### GET /search

  ?platform=PLATFORM - (optional) Platform identifier, youtube/spotify/soundcloud/etc
  &query=QUERY       - (required) The search query

Search for a song matching a query, optionally coming from a specific platform.

### POST /memo

body {
  platform: string - Platform identifier, youtube/spotify/soundcloud/etc
  id: string       - Song ID
  channel: string  - Channel name of the chat that requested this song
}

Memorize the song, allowing it to be returned from `/random`.