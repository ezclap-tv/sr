use super::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct PageInfo {
  #[serde(rename = "totalResults")]
  total_results: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct VideoList {
  pub items: Vec<VideoListItem>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct VideoListItem {
  pub id: String,
  pub snippet: VideoListItemSnippet,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct VideoListItemSnippet {
  #[serde(rename = "channelId")]
  pub channel_id: String,
  pub title: String,
  #[serde(rename = "publishedAt")]
  pub published_at: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct PlaylistItemList {
  pub items: Vec<PlaylistItem>,
  #[serde(rename = "nextPageToken")]
  pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct PlaylistItem {
  #[serde(rename = "contentDetails")]
  pub content_details: PlaylistItemContentDetails,
  pub status: PlaylistItemStatus,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct PlaylistItemContentDetails {
  #[serde(rename = "videoId")]
  pub video_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct PlaylistItemStatus {
  #[serde(rename = "privacyStatus")]
  pub privacy_status: PrivacyStatus,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum PrivacyStatus {
  #[serde(rename = "public")]
  Public,
  #[serde(rename = "unlisted")]
  Unlisted,
  #[serde(other)]
  Unspecified,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn deserialize_videos() {
    let data = r#"
        {
          "kind": "youtube#videoListResponse",
          "etag": "mS8DM8Dsd3_PSdw8tydsmBkhJpg",
          "items": [
            {
              "kind": "youtube#video",
              "etag": "kFGyuGlSrGmFEXBAJORgWejiNEQ",
              "id": "Ks-_Mh1QhMc",
              "snippet": {
                "publishedAt": "2012-10-01T15:27:35Z",
                "channelId": "UCAuUUnT6oDeKwE6v1NGQxug",
                "title": "Your body language may shape who you are | Amy Cuddy",
                "description": "Body language affects how others see us, but it may also change how we see ourselves. Social psychologist Amy Cuddy argues that \"power posing\" -- standing in a posture of confidence, even when we don't feel confident -- can boost feelings of confidence, and might have an impact on our chances for success. (Note: Some of the findings presented in this talk have been referenced in an ongoing debate among social scientists about robustness and reproducibility. Read Amy Cuddy's response here: http://ideas.ted.com/inside-the-debate-about-power-posing-a-q-a-with-amy-cuddy/)\n\nGet TED Talks recommended just for you! Learn more at https://www.ted.com/signup.\n\nThe TED Talks channel features the best talks and performances from the TED Conference, where the world's leading thinkers and doers give the talk of their lives in 18 minutes (or less). Look for talks on Technology, Entertainment and Design -- plus science, business, global issues, the arts and more.\n\nFollow TED on Twitter: http://www.twitter.com/TEDTalks\nLike TED on Facebook: https://www.facebook.com/TED\n\nSubscribe to our channel: https://www.youtube.com/TED",
                "thumbnails": {
                  "default": {
                    "url": "https://i.ytimg.com/vi/Ks-_Mh1QhMc/default.jpg",
                    "width": 120,
                    "height": 90
                  },
                  "medium": {
                    "url": "https://i.ytimg.com/vi/Ks-_Mh1QhMc/mqdefault.jpg",
                    "width": 320,
                    "height": 180
                  },
                  "high": {
                    "url": "https://i.ytimg.com/vi/Ks-_Mh1QhMc/hqdefault.jpg",
                    "width": 480,
                    "height": 360
                  },
                  "standard": {
                    "url": "https://i.ytimg.com/vi/Ks-_Mh1QhMc/sddefault.jpg",
                    "width": 640,
                    "height": 480
                  },
                  "maxres": {
                    "url": "https://i.ytimg.com/vi/Ks-_Mh1QhMc/maxresdefault.jpg",
                    "width": 1280,
                    "height": 720
                  }
                },
                "channelTitle": "TED",
                "tags": [
                  "Amy Cuddy",
                  "TED",
                  "TEDTalk",
                  "TEDTalks",
                  "TED Talk",
                  "TED Talks",
                  "TEDGlobal",
                  "brain",
                  "business",
                  "psychology",
                  "self",
                  "success"
                ],
                "categoryId": "22",
                "liveBroadcastContent": "none",
                "defaultLanguage": "en",
                "localized": {
                  "title": "Your body language may shape who you are | Amy Cuddy",
                  "description": "Body language affects how others see us, but it may also change how we see ourselves. Social psychologist Amy Cuddy argues that \"power posing\" -- standing in a posture of confidence, even when we don't feel confident -- can boost feelings of confidence, and might have an impact on our chances for success. (Note: Some of the findings presented in this talk have been referenced in an ongoing debate among social scientists about robustness and reproducibility. Read Amy Cuddy's response here: http://ideas.ted.com/inside-the-debate-about-power-posing-a-q-a-with-amy-cuddy/)\n\nGet TED Talks recommended just for you! Learn more at https://www.ted.com/signup.\n\nThe TED Talks channel features the best talks and performances from the TED Conference, where the world's leading thinkers and doers give the talk of their lives in 18 minutes (or less). Look for talks on Technology, Entertainment and Design -- plus science, business, global issues, the arts and more.\n\nFollow TED on Twitter: http://www.twitter.com/TEDTalks\nLike TED on Facebook: https://www.facebook.com/TED\n\nSubscribe to our channel: https://www.youtube.com/TED"
                },
                "defaultAudioLanguage": "en"
              }
            },
            {
              "kind": "youtube#video",
              "etag": "kFGyuGlSrGmFEXBAJORgWejiNEQ",
              "id": "Ks-_Mh1QhMc",
              "snippet": {
                "publishedAt": "2012-10-01T15:27:35Z",
                "channelId": "UCAuUUnT6oDeKwE6v1NGQxug",
                "title": "Your body language may shape who you are | Amy Cuddy",
                "description": "Body language affects how others see us, but it may also change how we see ourselves. Social psychologist Amy Cuddy argues that \"power posing\" -- standing in a posture of confidence, even when we don't feel confident -- can boost feelings of confidence, and might have an impact on our chances for success. (Note: Some of the findings presented in this talk have been referenced in an ongoing debate among social scientists about robustness and reproducibility. Read Amy Cuddy's response here: http://ideas.ted.com/inside-the-debate-about-power-posing-a-q-a-with-amy-cuddy/)\n\nGet TED Talks recommended just for you! Learn more at https://www.ted.com/signup.\n\nThe TED Talks channel features the best talks and performances from the TED Conference, where the world's leading thinkers and doers give the talk of their lives in 18 minutes (or less). Look for talks on Technology, Entertainment and Design -- plus science, business, global issues, the arts and more.\n\nFollow TED on Twitter: http://www.twitter.com/TEDTalks\nLike TED on Facebook: https://www.facebook.com/TED\n\nSubscribe to our channel: https://www.youtube.com/TED",
                "thumbnails": {
                  "default": {
                    "url": "https://i.ytimg.com/vi/Ks-_Mh1QhMc/default.jpg",
                    "width": 120,
                    "height": 90
                  },
                  "medium": {
                    "url": "https://i.ytimg.com/vi/Ks-_Mh1QhMc/mqdefault.jpg",
                    "width": 320,
                    "height": 180
                  },
                  "high": {
                    "url": "https://i.ytimg.com/vi/Ks-_Mh1QhMc/hqdefault.jpg",
                    "width": 480,
                    "height": 360
                  },
                  "standard": {
                    "url": "https://i.ytimg.com/vi/Ks-_Mh1QhMc/sddefault.jpg",
                    "width": 640,
                    "height": 480
                  },
                  "maxres": {
                    "url": "https://i.ytimg.com/vi/Ks-_Mh1QhMc/maxresdefault.jpg",
                    "width": 1280,
                    "height": 720
                  }
                },
                "channelTitle": "TED",
                "tags": [
                  "Amy Cuddy",
                  "TED",
                  "TEDTalk",
                  "TEDTalks",
                  "TED Talk",
                  "TED Talks",
                  "TEDGlobal",
                  "brain",
                  "business",
                  "psychology",
                  "self",
                  "success"
                ],
                "categoryId": "22",
                "liveBroadcastContent": "none",
                "defaultLanguage": "en",
                "localized": {
                  "title": "Your body language may shape who you are | Amy Cuddy",
                  "description": "Body language affects how others see us, but it may also change how we see ourselves. Social psychologist Amy Cuddy argues that \"power posing\" -- standing in a posture of confidence, even when we don't feel confident -- can boost feelings of confidence, and might have an impact on our chances for success. (Note: Some of the findings presented in this talk have been referenced in an ongoing debate among social scientists about robustness and reproducibility. Read Amy Cuddy's response here: http://ideas.ted.com/inside-the-debate-about-power-posing-a-q-a-with-amy-cuddy/)\n\nGet TED Talks recommended just for you! Learn more at https://www.ted.com/signup.\n\nThe TED Talks channel features the best talks and performances from the TED Conference, where the world's leading thinkers and doers give the talk of their lives in 18 minutes (or less). Look for talks on Technology, Entertainment and Design -- plus science, business, global issues, the arts and more.\n\nFollow TED on Twitter: http://www.twitter.com/TEDTalks\nLike TED on Facebook: https://www.facebook.com/TED\n\nSubscribe to our channel: https://www.youtube.com/TED"
                },
                "defaultAudioLanguage": "en"
              }
            }
          ],
          "pageInfo": {
            "totalResults": 2,
            "resultsPerPage": 2
          }
        }
      "#;

    assert_eq!(
      serde_json::from_str::<VideoList>(data).unwrap(),
      VideoList {
        items: vec![
          VideoListItem {
            id: "Ks-_Mh1QhMc".into(),
            snippet: VideoListItemSnippet {
              channel_id: "UCAuUUnT6oDeKwE6v1NGQxug".into(),
              title: "Your body language may shape who you are | Amy Cuddy".into(),
              published_at: DateTime::<Utc>::from(DateTime::parse_from_rfc3339("2012-10-01T15:27:35Z").unwrap())
            }
          },
          VideoListItem {
            id: "Ks-_Mh1QhMc".into(),
            snippet: VideoListItemSnippet {
              channel_id: "UCAuUUnT6oDeKwE6v1NGQxug".into(),
              title: "Your body language may shape who you are | Amy Cuddy".into(),
              published_at: DateTime::<Utc>::from(DateTime::parse_from_rfc3339("2012-10-01T15:27:35Z").unwrap())
            }
          }
        ],
      }
    )
  }

  #[test]
  fn deserialize_playlist_items() {
    let data = r#"
        {
          "kind": "youtube#playlistItemListResponse",
          "etag": "m6PJ_s7BOULkhCMdNyYGNcZSZf8",
          "nextPageToken": "EAAaBlBUOkNBSQ",
          "items": [
            {
              "kind": "youtube#playlistItem",
              "etag": "NLlgFBuBjY5z0LnUUfu0gox3oak",
              "id": "UExISFl1bzh3UHhVeGFfeUhJQjNKZThSc056RFAwOHV1dC41NkI0NEY2RDEwNTU3Q0M2",
              "contentDetails": {
                "videoId": "kOCxHu_F5xo",
                "videoPublishedAt": "2016-01-21T13:36:16Z"
              },
              "status": {
                "privacyStatus": "public"
              }
            },
            {
              "kind": "youtube#playlistItem",
              "etag": "jtm2g5SSwCzqJUtilqEUvuS1DBo",
              "id": "UExISFl1bzh3UHhVeGFfeUhJQjNKZThSc056RFAwOHV1dC4yODlGNEE0NkRGMEEzMEQy",
              "contentDetails": {
                "videoId": "NdqbI0_0GsM",
                "videoPublishedAt": "2018-07-24T01:12:47Z"
              },
              "status": {
                "privacyStatus": "public"
              }
            }
          ],
          "pageInfo": {
            "totalResults": 539,
            "resultsPerPage": 2
          }
      }
      "#;

    assert_eq!(
      serde_json::from_str::<PlaylistItemList>(data).unwrap(),
      PlaylistItemList {
        items: vec![
          PlaylistItem {
            content_details: PlaylistItemContentDetails {
              video_id: "kOCxHu_F5xo".into()
            },
            status: PlaylistItemStatus {
              privacy_status: PrivacyStatus::Public
            }
          },
          PlaylistItem {
            content_details: PlaylistItemContentDetails {
              video_id: "NdqbI0_0GsM".into()
            },
            status: PlaylistItemStatus {
              privacy_status: PrivacyStatus::Public
            }
          }
        ],
        next_page_token: Some("EAAaBlBUOkNBSQ".into())
      }
    )
  }
}
