mod schema;

use super::Client;
use crate::common::util::query_ext::QueryExt;
use chrono::{DateTime, Utc};
use serde::Deserialize;

// TODO: write tests

pub struct YouTube {
  inner: reqwest::Client,
  base_url: String,
  api_key: String,
}

impl YouTube {
  pub fn new(
    base_url: impl Into<String>,
    api_key: impl Into<String>,
  ) -> Client<Self> {
    Client::new(Self {
      inner: reqwest::Client::new(),
      base_url: base_url.into(),
      api_key: api_key.into(),
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Video {
  pub id: String,
  pub title: String,
  pub channel_id: String,
  pub published_at: DateTime<Utc>,
}

impl YouTube {
  pub async fn playlist_videos(
    self: Client<Self>,
    playlist_id: &str,
  ) -> reqwest::Result<Vec<Video>> {
    // base requests - some of the parameters are shared, so we re-use them by cloning these
    let playlist_base_request = self
      .inner
      .get(format!("{}/playlistItems", self.base_url))
      .query(&[
        ("key", self.api_key.as_str()),
        ("part", "contentDetails,status"),
        ("maxResults", "50"),
        ("playlistId", playlist_id),
      ]);
    let video_base_request = self
      .inner
      .get(format!("{}/videos", self.base_url))
      .query(&[("key", self.api_key.as_str()), ("part", "snippet")]);

    let mut result = vec![];
    let mut page_token = Option::<String>::None;
    loop {
      // 1. fetch playlist items
      let playlist_items = playlist_base_request
        .try_clone()
        .unwrap()
        .query_opt(page_token.as_ref().map(|t| ("pageToken", t.as_str())))
        .send()
        .await?
        .json::<schema::PlaylistItemList>()
        .await?;
      // 2. fetch videos
      let videos = video_base_request
        .try_clone()
        .unwrap()
        .query_iter(
          playlist_items
            .items
            .iter()
            .map(|item| ("id", item.content_details.video_id.as_str())),
        )
        .send()
        .await?
        .json::<schema::VideoList>()
        .await?;
      // 3. store videos
      result.extend(videos.items.into_iter().map(|video| Video {
        id: video.id,
        title: video.snippet.title,
        channel_id: video.snippet.channel_id,
        published_at: video.snippet.published_at,
      }));
      // 4. paginate playlist items
      if let Some(next_page_token) = playlist_items.next_page_token {
        page_token = Some(next_page_token);
      } else {
        break;
      }
    }
    Ok(result)
  }
}
