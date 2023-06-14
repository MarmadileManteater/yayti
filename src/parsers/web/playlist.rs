
use serde_json::{Map,Value,json};
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
  pub videos: Option<Vec::<PlaylistVideo>>,
  pub continuation: Option<String>
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

pub fn parse(browse: &Value, lang: &str) -> Playlist {
  Playlist {
    title: match browse["header"]["playlistHeaderRenderer"]["title"]["simpleText"].as_str() {
      Some(simple_text) => Some(String::from(simple_text)),
      None => None
    },
    playlist_id: match browse["header"]["playlistHeaderRenderer"]["playlistId"].as_str() {
      Some(playlist_id) => Some(String::from(playlist_id)),
      None => None
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
      None => None
    },
    author_id: match browse["header"]["playlistHeaderRenderer"]["ownerText"]["runs"][0]["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str() {
      Some(text) => Some(String::from(text)),
      None => None
    },
    author_url: match browse["header"]["playlistHeaderRenderer"]["ownerText"]["runs"][0]["navigationEndpoint"]["browseEndpoint"]["canonicalBaseUrl"].as_str() {
      Some(text) => Some(String::from(text)),
      None => None
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
      None => None
    },
    view_count: match browse["header"]["playlistHeaderRenderer"]["viewCountText"]["simpleText"].as_str() {
      Some(view_count_text) => {
        let match_numbers = Regex::new(r"[0-9]+").unwrap();
        let view_numbers_string = match_numbers.find_iter(&view_count_text).map(|m| m.as_str()).collect::<String>();
        match i32::from_str(&view_numbers_string) {
          Ok(number) => Some(number),
          Err(_) => None
        }
      },
      None => None
    },
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
                    Some(PlaylistVideo {
                      title: match content["playlistVideoRenderer"]["title"]["runs"].as_array() {
                        Some(runs) => Some(runs.into_iter().map(|run| run["text"].as_str().unwrap_or("")).collect::<String>()),
                        None => None
                      },
                      video_id: match content["playlistVideoRenderer"]["videoId"].as_str() {
                        Some(video_id) => Some(String::from(video_id)),
                        None => None
                      },
                      author: match content["playlistVideoRenderer"]["shortBylineText"]["runs"].as_array() {
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
                      author_id: match content["playlistVideoRenderer"]["shortBylineText"]["runs"].as_array() {
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
                      video_thumbnails: match content["playlistVideoRenderer"]["thumbnail"]["thumbnails"].as_array() {
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
                      index: match content["playlistVideoRenderer"]["index"]["simpleText"].as_str() {
                        Some(index) => match i32::from_str(index) {
                          Ok(num) => Some(num),
                          Err(_) => None
                        },
                        None => None
                      },
                      length_seconds: match content["playlistVideoRenderer"]["lengthSeconds"].as_str() {
                        Some(seconds) => match i32::from_str(seconds) {
                          Ok(num) => Some(num),
                          Err(_) => None
                        },
                        None => None
                      }
                    })
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
        videos
      }
      None => None
    },
    continuation: None // todo implement
  }
}
