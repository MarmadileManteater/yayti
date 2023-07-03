
use serde_json::{Map,Value,json};
use substring::Substring;
use std::str::FromStr;
use regex::Regex;
use crate::helpers::{AuthorThumbnail, Thumbnail};
#[cfg(feature = "parse_languages_to_published")]
use crate::helpers::parse_date_to_published;

#[derive(Clone)]
pub struct PlaylistVideo {
  pub title: Option<String>,
  pub video_id: Option<String>,
  pub author: Option<String>,
  pub author_id: Option<String>,
  pub video_thumbnails: Option<Vec::<AuthorThumbnail>>,
  pub index: Option<i32>,
  pub length_seconds: Option<i32>
}

impl PlaylistVideo {
  pub fn into_inv(&self) -> Map::<String, Value> {
    let mut j_object = Map::<String, Value>::new();
    j_object.insert(String::from("title"), json!(self.title));
    j_object.insert(String::from("videoId"), json!(self.video_id));
    j_object.insert(String::from("author"), json!(self.author));
    j_object.insert(String::from("authorId"), json!(self.author_id));
    j_object.insert(String::from("authorUrl"), json!(format!("/channel/{}", self.author_id.to_owned().unwrap_or(String::from("")))));
    j_object.insert(String::from("videoThumbnails"), json!(self.video_thumbnails));
    j_object.insert(String::from("index"), json!(self.index));
    j_object.insert(String::from("lengthSeconds"), json!(self.length_seconds));
    j_object
  }
}

#[derive(Clone)]
pub struct Playlist {
  pub title: Option<String>,
  pub playlist_id: Option<String>,
  pub playlist_thumbnails: Option<Vec::<AuthorThumbnail>>,
  pub author: Option<String>,
  pub author_id: Option<String>,
  pub author_url: Option<String>,
  pub author_thumbnails: Option<Vec::<AuthorThumbnail>>,
  pub description: Option<String>,
  pub description_html: Option<String>,
  pub video_count: Option<i32>,
  pub view_count: Option<i32>,
  pub updated: Option<i64>,
  pub is_listed: Option<bool>,
  pub videos: Option<Vec::<PlaylistVideo>>
}

pub struct Alert {
  pub alert_type: Option<String>,
  pub alert_text: Option<String>
}

pub struct PlaylistError {
  pub alerts: Vec::<Alert>
}

impl Playlist {
  pub fn into_inv(&self) -> Map::<String, Value> {
    let mut j_object = Map::<String, Value>::new();
    j_object.insert(String::from("title"), json!(self.title));
    j_object.insert(String::from("playlistId"), json!(self.playlist_id));
    j_object.insert(String::from("playlistThumbnail"), json!(match &self.playlist_thumbnails {
      Some(thumbnails) => {
        Some(thumbnails[thumbnails.len() - 1].to_owned())
      },
      None => {
        None
      }
    }));
    
    j_object.insert(String::from("author"), json!(self.author));
    j_object.insert(String::from("authorId"), json!(self.author_id));
    j_object.insert(String::from("authorUrl"), json!(format!("/channel/{}", self.author_id.to_owned().unwrap_or(String::from("")))));
    j_object.insert(String::from("authorThumbnails"), json!(self.author_thumbnails));
    j_object.insert(String::from("description"), json!(self.description));
    j_object.insert(String::from("descriptionHtml"), json!(self.description_html));
    j_object.insert(String::from("videoCount"), json!(self.video_count));
    j_object.insert(String::from("viewCount"), json!(self.view_count));
    j_object.insert(String::from("updated"), json!(self.updated));
    j_object.insert(String::from("isListed"), json!(self.is_listed));
    j_object.insert(String::from("videos"), self.videos.to_owned().unwrap_or(vec!()).into_iter().map(|video| video.into_inv()).collect::<Value>());
    j_object
  }
}

fn parse_playlist_video_renderer(renderer: &Value) -> PlaylistVideo {
  PlaylistVideo {
    title: match renderer["title"]["runs"].as_array() {
      Some(runs) => Some(runs.into_iter().map(|run| run["text"].as_str().unwrap_or("")).collect::<String>()),
      None => None
    },
    video_id: match renderer["videoId"].as_str() {
      Some(video_id) => Some(String::from(video_id)),
      None => None
    },
    author: match renderer["shortBylineText"]["runs"].as_array() {
      Some(runs) => {
        if runs.len() > 0 {
          match runs[0]["text"].as_str() {
            Some(text) => Some(String::from(text)),
            None => None
          }
        } else {
          None
        }
      },
      None => None
    },
    author_id: match renderer["shortBylineText"]["runs"].as_array() {
      Some(runs) => {
        if runs.len() > 0 {
          match runs[0]["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str() {
            Some(text) => Some(String::from(text)),
            None => None
          }
        } else {
          None
        }
      },
      None => None
    },
    video_thumbnails: match renderer["thumbnail"]["thumbnails"].as_array() {
      Some(thumbnails) => {
        Some(thumbnails.into_iter().filter_map(|thumbnail| {
          match (thumbnail["url"].as_str(), thumbnail["width"].as_i64(), thumbnail["height"].as_i64()) {
            (Some(url), Some(width), Some(height)) => {
              Some(AuthorThumbnail {
                url: String::from(url),
                width: width as i32,
                height: height as i32
              })
            },
            _ => {
              None
            }
          }
        }).collect::<Vec::<AuthorThumbnail>>())
      },
      None => None
    },
    index: match renderer["index"]["simpleText"].as_str() {
      Some(index) => match i32::from_str(index) {
        Ok(num) => Some(num),
        Err(_) => None
      },
      None => None
    },
    length_seconds: match renderer["lengthSeconds"].as_str() {
      Some(seconds) => match i32::from_str(seconds) {
        Ok(num) => Some(num),
        Err(_) => None
      },
      None => None
    }
  }
} 

pub fn parse(browse: &Value, lang: &str) -> Result<Playlist, PlaylistError> {
  // return playlist error if alerts have any entries
  match browse["alerts"].as_array() {
    Some(alerts) => {
      let alerts = alerts.into_iter().map(|alert| {
        let alert_type = alert["alertRenderer"]["type"].as_str().map(|str| String::from(str));
        let alert_text = alert["alertRenderer"]["text"]["runs"].as_array().map(|runs| {
          runs.into_iter().filter_map(|value| value["text"].as_str()).collect::<String>()
        });
        Alert {
          alert_text,
          alert_type
        }
      }).collect::<Vec<Alert>>();
      return Err(PlaylistError { alerts });
    },
    None => {}
  };
  Ok(Playlist {
    title: match browse["header"]["playlistHeaderRenderer"]["title"]["simpleText"].as_str() {
      Some(simple_text) => Some(String::from(simple_text)),
      None => browse["metadata"]["playlistMetadataRenderer"]["title"].as_str().map(|str| String::from(str))
    },
    playlist_id: match browse["header"]["playlistHeaderRenderer"]["playlistId"].as_str() {
      Some(playlist_id) => Some(String::from(playlist_id)),
      None => browse["responseContext"]["serviceTrackingParams"][0]["params"][0]["value"].as_str().map(|str| {
        String::from(str.substring(2, str.len()))
      })
    },
    playlist_thumbnails: match browse["header"]["playlistHeaderBanner"]["heroPlaylistThumbnailRenderer"]["thumbnail"]["thumbnails"].as_array() {
      Some(thumbnails) => {
        Some(thumbnails.into_iter().filter_map(|thumbnail| {
          match (thumbnail["url"].as_str(), thumbnail["width"].as_i64(), thumbnail["height"].as_i64()) {
            (Some(url), Some(width), Some(height)) => {
              Some(AuthorThumbnail {
                url: String::from(url),
                width: width as i32,
                height: height as i32
              })
            },
            _ => {
              None
            }
          }
        }).collect::<Vec::<AuthorThumbnail>>())
      },
      None => None
    },
    author: match browse["header"]["playlistHeaderRenderer"]["ownerText"]["runs"][0]["text"].as_str() {
      Some(text) => Some(String::from(text)),
      None => browse["sidebar"]["playlistSidebarRenderer"]["items"][1]["playlistSidebarSecondaryInfoRenderer"]["videoOwner"]["videoOwnerRenderer"]["title"]["runs"][0]["text"].as_str().map(String::from)
    },
    author_id: match browse["header"]["playlistHeaderRenderer"]["ownerText"]["runs"][0]["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str() {
      Some(text) => Some(String::from(text)),
      None => browse["sidebar"]["playlistSidebarRenderer"]["items"][1]["playlistSidebarSecondaryInfoRenderer"]["videoOwner"]["videoOwnerRenderer"]["title"]["runs"][0]["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str().map(String::from)
    },
    author_url: match browse["header"]["playlistHeaderRenderer"]["ownerText"]["runs"][0]["navigationEndpoint"]["browseEndpoint"]["canonicalBaseUrl"].as_str() {
      Some(text) => Some(String::from(text)),
      None => browse["sidebar"]["playlistSidebarRenderer"]["items"][1]["playlistSidebarSecondaryInfoRenderer"]["videoOwner"]["videoOwnerRenderer"]["title"]["runs"][0]["navigationEndpoint"]["browseEndpoint"]["canonicalBaseUrl"].as_str().map(String::from)
    },
    author_thumbnails: match browse["sidebar"]["playlistSidebarRenderer"]["items"].as_array() {
      Some(side_bar_items) => {
        let mut thumbnails = None::<Vec::<AuthorThumbnail>>;
        for item in side_bar_items {
          thumbnails = match item["playlistSidebarSecondaryInfoRenderer"]["videoOwner"]["videoOwnerRenderer"]["thumbnail"]["thumbnails"].as_array() {
            Some(thumbnails) => {
              Some(thumbnails.into_iter().filter_map(|thumbnail| {
                match (thumbnail["url"].as_str(), thumbnail["width"].as_i64(), thumbnail["height"].as_i64()) {
                  (Some(url), Some(width), Some(height)) => {
                    Some(AuthorThumbnail {
                      url: String::from(url),
                      width: width as i32,
                      height: height as i32
                    })
                  },
                  _ => {
                    None
                  }
                }
              }).collect::<Vec::<AuthorThumbnail>>())
            },
            None => None
          };
          match thumbnails {
            Some(_) => break,
            None => continue
          };
        }
        thumbnails
      }
      None => None
    },
    description: match browse["microformat"]["microformatDataRenderer"]["description"].as_str() {
      Some(description) => Some(String::from(description)),
      None => None
    },
    description_html: None,//todo implement (first I have to find a playlist with a description)
    video_count: match browse["header"]["playlistHeaderRenderer"]["stats"][0]["runs"].as_array() {
      Some(stats) => {
        if stats.len() > 0 {
          match i32::from_str(stats[0]["text"].as_str().unwrap_or("")) {
            Ok(num) => Some(num),
            Err(_) => None
          }
        } else {
          None
        }
      },
      None => browse["sidebar"]["playlistSidebarRenderer"]["items"][0]["playlistSidebarPrimaryInfoRenderer"]["stats"][0]["runs"][0]["text"].as_str().map(|str| {
        match i32::from_str(str) {
          Ok(num) => Some(num),
          Err(_) => None
        }
      }).unwrap_or(None)
    },
    view_count: {
      let view_count_text = match browse["header"]["playlistHeaderRenderer"]["viewCountText"]["simpleText"].as_str() {
        Some(view_count_text) => {
          Some(view_count_text)
        },
        None => match browse["sidebar"]["playlistSidebarRenderer"]["items"][0]["playlistSidebarPrimaryInfoRenderer"]["stats"][1]["simpleText"].as_str() {
          Some(view_count_text) => {
            Some(view_count_text)
          },
          None => None
        }
      };
      match view_count_text {
        Some(view_count_text) => {
          let match_numbers = Regex::new(r"[0-9]+").unwrap();
          let view_numbers_string = match_numbers.find_iter(&view_count_text).map(|m| m.as_str()).collect::<String>();
          match i32::from_str(&view_numbers_string) {
            Ok(number) => Some(number),
            Err(_) => None
          }
        },
        None => None
      }
    },
    // this only works for updates made like x number of days ago
    // before a certain time frame yt likes to use relative dates like "5 days ago" which makes this harder
    updated: match browse["sidebar"]["playlistSidebarRenderer"]["items"][0]["playlistSidebarPrimaryInfoRenderer"]["stats"][2]["runs"][1]["text"].as_str() {
      Some(date_string) => {
        #[cfg(feature = "parse_languages_to_published")]
        match parse_date_to_published(lang, &crate::helpers::ParseDateOption::ParseFullDate(String::from(date_string))) {
          Ok(date) => {
            Some(date)
          },
          Err(_) => {
            None
          }
        }
        // not really easy to do this without a map of language data
        // todo✏ add basic english language date parsing as a fallback for no language map feature
        #[cfg(not(feature = "parse_languages_to_published"))]
        None
      },
      None => None
    },// todo move the logic for parsing dates with lang info to somewhere common
    is_listed: match browse["microformat"]["microformatDataRenderer"]["unlisted"].as_bool() {
      Some(unlisted) => Some(!unlisted),
      None => None
    },
    videos: match browse["contents"]["twoColumnBrowseResultsRenderer"]["tabs"].as_array() {
      Some(tabs) => {
        let mut videos = None;
        for tab in tabs {
          videos = match tab["tabRenderer"]["content"]["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"][0]["playlistVideoListRenderer"]["contents"].as_array() {
            Some(contents) => {
              Some(contents.into_iter().filter_map(|content| {
                match content["playlistVideoRenderer"].as_object() {
                  Some(_) => {
                    Some(parse_playlist_video_renderer(&content["playlistVideoRenderer"]))
                  },
                  None => None
                }
              }).collect::<Vec::<PlaylistVideo>>())
            },
            None => {
              None
            }
          };
          match videos {
            Some(_) => break,
            None => continue
          }
        }
        match videos {
          Some(_) => videos,
          None => match browse["onResponseReceivedActions"][0]["appendContinuationItemsAction"]["continuationItems"].as_array() {
            Some(continuation_items) => {
              Some(continuation_items.into_iter().map(|continuation_item| {
                parse_playlist_video_renderer(&continuation_item["playlistVideoRenderer"])
              }).collect::<Vec::<PlaylistVideo>>())
            },
            None => None
          }
        }
      },
      None => None
    }
  })
}
