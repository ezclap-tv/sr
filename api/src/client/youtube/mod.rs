mod schema;

use crate::common::util::query_ext::QueryExt;
use chrono::{DateTime, Utc};
use secrecy::{ExposeSecret, Secret};
use std::sync::Arc;

pub struct Client {
  inner: reqwest::Client,
  base_url: String,
  api_key: Secret<String>,
}

impl Client {
  pub fn new(
    base_url: impl Into<String>,
    api_key: Secret<String>,
  ) -> Arc<Client> {
    Arc::new(Self {
      inner: reqwest::Client::new(),
      base_url: base_url.into(),
      api_key,
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

impl Client {
  pub async fn playlist_videos(
    self: Arc<Self>,
    playlist_id: &str,
  ) -> reqwest::Result<Vec<Video>> {
    let mut result = vec![];
    let mut page_token = Option::<String>::None;
    loop {
      // 1. fetch playlist items
      let playlist_items = self
        .inner
        .get(format!("{}/playlistItems", self.base_url))
        .query(&[
          ("key", self.api_key.expose_secret().as_str()),
          ("part", "contentDetails,status"),
          ("maxResults", "50"),
          ("playlistId", playlist_id),
        ])
        .query_opt("pageToken", page_token.as_ref())
        .send()
        .await?
        .json::<schema::PlaylistItemList>()
        .await?;
      // 2. fetch videos
      let videos = self
        .inner
        .get(format!("{}/videos", self.base_url))
        .query(&[
          ("key", self.api_key.expose_secret().as_str()),
          ("part", "snippet"),
        ])
        .query_iter(
          "id",
          playlist_items
            .items
            .iter()
            .filter(|item| {
              item.status.privacy_status != schema::PrivacyStatus::Unspecified
            })
            .map(|v| &v.content_details.video_id),
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

#[cfg(test)]
mod tests {
  use super::*;

  use wiremock::{
    matchers::{method, path},
    Mock, MockServer, Request, ResponseTemplate,
  };

  fn videos_response(r: &Request) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(schema::VideoList {
      items: r
        .url
        .query_pairs()
        .filter_map(|(k, v)| if k == "id" { Some(v) } else { None })
        .map(|i| schema::VideoListItem {
          snippet: schema::VideoListItemSnippet {
            channel_id: "test".into(),
            title: format!("{i} title"),
            published_at: Utc::now(),
          },
          id: i.into(),
        })
        .collect(),
    })
  }

  #[actix_rt::test]
  async fn playlist_happy_path() -> anyhow::Result<()> {
    let mock = MockServer::start().await;

    Mock::given(path("/playlistItems"))
      .and(method("GET"))
      .respond_with(|_r: &Request| {
        ResponseTemplate::new(200).set_body_json(schema::PlaylistItemList {
          items: (0..25)
            .into_iter()
            .map(|i| schema::PlaylistItem {
              content_details: schema::PlaylistItemContentDetails {
                video_id: format!("video{i}"),
              },
              status: schema::PlaylistItemStatus {
                privacy_status: schema::PrivacyStatus::Public,
              },
            })
            .collect(),
          next_page_token: None,
        })
      })
      .expect(1)
      .named("playlist_items")
      .mount(&mock)
      .await;
    Mock::given(path("/videos"))
      .and(method("GET"))
      .respond_with(videos_response)
      .expect(1)
      .named("videos")
      .mount(&mock)
      .await;

    let client = Client::new(mock.uri(), Secret::new("test".into()));
    assert_eq!(client.playlist_videos("test").await?.len(), 25);

    Ok(())
  }

  #[actix_rt::test]
  async fn playlist_filter_privacy_status_unspecified() -> anyhow::Result<()> {
    let mock = MockServer::start().await;

    Mock::given(path("/playlistItems"))
      .and(method("GET"))
      .respond_with(|_r: &Request| {
        ResponseTemplate::new(200).set_body_json(schema::PlaylistItemList {
          items: (0..50)
            .into_iter()
            .map(|i| schema::PlaylistItem {
              content_details: schema::PlaylistItemContentDetails {
                video_id: format!("video{i}"),
              },
              status: schema::PlaylistItemStatus {
                privacy_status: if i < 25 {
                  schema::PrivacyStatus::Public
                } else {
                  schema::PrivacyStatus::Unspecified
                },
              },
            })
            .collect(),
          next_page_token: None,
        })
      })
      .expect(1)
      .named("playlist_items")
      .mount(&mock)
      .await;
    Mock::given(path("/videos"))
      .and(method("GET"))
      .respond_with(videos_response)
      .expect(1)
      .named("videos")
      .mount(&mock)
      .await;

    let client = Client::new(mock.uri(), Secret::new("test".into()));
    assert_eq!(client.playlist_videos("test").await?.len(), 25);

    Ok(())
  }

  #[actix_rt::test]
  async fn playlist_handles_paging() -> anyhow::Result<()> {
    let mock = MockServer::start().await;

    Mock::given(path("/playlistItems"))
      .and(method("GET"))
      .respond_with(|r: &Request| {
        let has_page_token = r
          .url
          .query()
          .map(|q| q.contains("pageToken"))
          .unwrap_or(false);
        let id_start = if has_page_token { 25 } else { 0 };
        ResponseTemplate::new(200).set_body_json(schema::PlaylistItemList {
          items: (id_start..id_start + 25)
            .into_iter()
            .map(|i| schema::PlaylistItem {
              content_details: schema::PlaylistItemContentDetails {
                video_id: format!("video{i}"),
              },
              status: schema::PlaylistItemStatus {
                privacy_status: schema::PrivacyStatus::Public,
              },
            })
            .collect(),
          next_page_token: if has_page_token {
            None
          } else {
            Some("0".into())
          },
        })
      })
      .expect(2)
      .named("playlist_items")
      .mount(&mock)
      .await;
    Mock::given(path("/videos"))
      .and(method("GET"))
      .respond_with(videos_response)
      .expect(2)
      .named("videos")
      .mount(&mock)
      .await;

    let client = Client::new(mock.uri(), Secret::new("test".into()));
    let response = client.playlist_videos("test").await?;
    assert_eq!(response.len(), 50);

    Ok(())
  }
}
