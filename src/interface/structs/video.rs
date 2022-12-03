
use super::super::innertube::ClientContext;
use super::super::helpers::generate_yt_video_thumbnails;
use super::super::super::constants::WEBSITE_BASE_URL;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use regex::Regex;
use chrono::prelude::*;
use log::warn;

#[derive(Debug, Deserialize, Serialize)]
pub struct Caption {
  pub label: String,
  pub language_code: String,
  pub url: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Size {
  pub name: String,
  pub label: String,
  pub width: i32,
  pub height: i32
}

impl Size {
  fn new(width: i32, height: i32) -> Size {
    Size {
      name: String::from(""),
      label: String::from(""),
      width: width,
      height: height
    }
  }
  fn fmt_inv(self) -> String {
    format!("{}x{}", self.width, self.height)
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorThumbnail {
  pub url: String,
  pub width: i32,
  pub height: i32
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Thumbnail {
  pub quality: String,
  pub url: String,
  pub width: i32,
  pub height: i32
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct Range {
  pub start: i32,
  pub end: i32
}

impl Range {
  fn fmt_inv(self) -> String {
    format!("{}-{}", self.start, self.end)
  }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AdaptiveFormat {
  pub init: Range,
  pub index: Range,
  pub bitrate: i32,
  pub url: String,
  pub itag: i32,
  pub mime_type: String,
  pub content_length: i32,
  pub last_modified_time: i64,
  pub projection_type: String,
  pub fps: Option<i32>,
  pub container: String,
  pub encoding: String,
  pub quality_label: Option<String>,
  pub audio_quality: Option<String>,
  pub audio_sample_rate: Option<i32>,
  pub audio_channels: Option<i32>
}

impl AdaptiveFormat {
  fn fmt_inv(self) -> serde_json::Map::<String, Value> {
    let mut j_object = serde_json::Map::new();
    let init_str = self.init.fmt_inv();
    if init_str != "0-0" {
      j_object.insert(String::from("init"), json!(init_str));
    } else {
      // Just set this to the max range if the range is empty
      j_object.insert(String::from("init"), json!("0-"));
    }
    let index_str = self.index.fmt_inv();
    if index_str != "0-0" {
      j_object.insert(String::from("index"), json!(index_str));
    } else {
      // Just set this to the max range if the range is empty
      j_object.insert(String::from("index"), json!("0-"));
    }
    j_object.insert(String::from("bitrate"), json!(self.bitrate));
    j_object.insert(String::from("url"), json!(self.url));
    j_object.insert(String::from("itag"), json!(self.itag));
    j_object.insert(String::from("type"), json!(self.mime_type));
    j_object.insert(String::from("clen"), json!(self.content_length));
    j_object.insert(String::from("lmt"), json!(self.last_modified_time));
    j_object.insert(String::from("projectionType"), json!(self.projection_type));
    match self.fps {
      Some(fps) => {
        j_object.insert(String::from("fps"), json!(fps));
      },
      None => {}
    };
    j_object.insert(String::from("container"), json!(self.container));
    j_object.insert(String::from("encoding"), json!(self.encoding));
    match self.quality_label {
      Some(quality_label) => {
        j_object.insert(String::from("qualityLabel"), json!(quality_label));
        j_object.insert(String::from("resolution"), json!(quality_label));
      },
      None => {}
    }
    match self.audio_quality {
      Some(audio_quality) => {
        j_object.insert(String::from("audioQuality"), json!(audio_quality));
      },
      None => {}
    }
    match self.audio_sample_rate {
      Some(audio_sample_rate) => {
        j_object.insert(String::from("audioSampleRate"), json!(audio_sample_rate));
      },
      None => {}
    }
    match self.audio_channels {
      Some(audio_channels) => {
        j_object.insert(String::from("audioChannels"), json!(audio_channels));
      },
      None => {}
    }
    j_object
  }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FormatStream {
  pub url: String,
  pub itag: i32,
  pub mime_type: String,
  pub quality: String,
  pub fps: i32,
  pub container: String,
  pub encoding: String,
  pub quality_label: String,
  pub size: Size
}

impl FormatStream {
  fn fmt_inv(self) -> serde_json::Map::<String, Value> {
    let mut json_object = serde_json::Map::new();
    json_object.insert(String::from("url"), json!(self.url));
    json_object.insert(String::from("itag"), json!(format!("{}", self.itag)));
    json_object.insert(String::from("type"), json!(self.mime_type));
    json_object.insert(String::from("quality"), json!(self.quality));
    json_object.insert(String::from("fps"), json!(self.fps));
    json_object.insert(String::from("container"), json!(self.container));
    json_object.insert(String::from("resolution"), json!(self.quality_label));
    json_object.insert(String::from("qualityLabel"), json!(self.quality_label));
    json_object.insert(String::from("size"), json!(self.size.fmt_inv()));
    json_object
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecommendedVideo {
  pub title: String,
  pub video_id: String,
  pub video_thumbnails: Vec<Thumbnail>,
  pub author: String,
  pub author_id: String,
  pub published_text: String,
  pub view_count: i32,
  pub view_count_text: String,
  pub length_seconds: i32
}

impl RecommendedVideo {
  // Parse a recommended video from a compact video renderer
  pub fn from_compact_video_renderer(j_object: &Value) -> RecommendedVideo {
    let title = String::from(match j_object["title"]["simpleText"].as_str() {
      Some(title) => title,
      None => ""
    });
    let video_id = String::from(match j_object["videoId"].as_str() {
      Some(video_id) => video_id,
      None => ""
    });
    let thumbnails = generate_yt_video_thumbnails(&video_id);
    let author = String::from(match j_object["longBylineText"]["runs"][0]["text"].as_str() {
      Some(author) => author,
      None => ""
    });
    let author_id = String::from(match j_object["longBylineText"]["runs"][0]["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str() {
      Some(author_id) => author_id,
      None => ""
    });
    let published_time_text = String::from(match j_object["publishedTimeText"]["simpleText"].as_str() {
      Some(published_time_text) => published_time_text,
      None => ""
    });
    let view_count_text = String::from(match j_object["viewCountText"]["simpleText"].as_str() {
      Some(view_count_text) => view_count_text,
      None => ""
    });
    let view_count = match view_count_text.replace(",", "").replace(" views", "").as_str().parse::<i32>() {
      Ok(view_count) => view_count,
      Err(_err) => 0
    };
    let length_text = String::from(match j_object["lengthText"]["simpleText"].as_str() {
      Some(length_text) => length_text,
      None => ""
    });
    // 🧮Calculate the length seconds from the length_text
    let mut magnitude_multiplier;
    let length_seconds = match Regex::new(r"([0-9]*):*([0-9]*):([0-9]*)").unwrap().captures(&length_text) {
      Some(captures) => {
        let mut total = 0;
        let base: i32 = 60;
        // If the length text has more than 5 characters in it, it must contain hours.
        let pow = (if length_text.len() > 5 { 2 } else { 1 }) as u32;
        // The magnitude multiplier contains the current multiplier relative to seconds (ex: 3600 = hours, 60 = minutes, 1 = seconds)
        magnitude_multiplier = base.pow(pow);
        for i in 1..captures.len() {
          match captures[i].parse::<i32>() {
            Ok(capture) => {
              total += capture * magnitude_multiplier;
              // Reduce the magnitude multiplier on each iteration
              magnitude_multiplier = magnitude_multiplier / 60
            },
            Err(_err) => {}
          }
        }
        total
      },
      None => 0
    };
    RecommendedVideo {
      title: title,
      video_id: video_id,
      video_thumbnails: thumbnails,
      author: author,
      author_id: author_id,
      published_text: published_time_text,
      view_count: view_count,
      view_count_text: view_count_text,
      length_seconds: length_seconds as i32
    }
  }
  fn fmt_inv(self) -> serde_json::Map::<String, Value> {
    let mut j_object = serde_json::Map::new();
    j_object.insert(String::from("videoId"), json!(self.video_id));
    j_object.insert(String::from("title"), json!(self.title));
    j_object.insert(String::from("videoThumbnails"), self.video_thumbnails.iter().map(|thumbnail| {
      let thumbnail_string = match serde_json::to_string_pretty(thumbnail) {
        Ok(thumbnail_string) => thumbnail_string,
        Err(_error) => String::from("[]")
      };
      match serde_json::from_str(&thumbnail_string) {
        Ok(thumbnail_value) => thumbnail_value,
        Err(_err) => Value::default()
      }
    }).collect());
    j_object.insert(String::from("author"), json!(self.author));
    j_object.insert(String::from("authorUrl"), json!(format!("/channel/{}", self.author_id)));
    j_object.insert(String::from("authorId"), json!(self.author_id));
    j_object.insert(String::from("lengthSeconds"), json!(self.length_seconds));
    j_object.insert(String::from("viewCountText"), json!(self.view_count_text));
    j_object.insert(String::from("viewCount"), json!(self.view_count));
    j_object
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Video {
  pub title: String,
  pub video_id: String,
  pub video_thumbnails: Vec<Thumbnail>,
  pub storyboards: Vec<Value>,
  pub description: String,
  pub description_html: String,
  pub published: i32,
  pub published_text: String,
  pub keywords: Vec<String>,
  pub view_count: i32,
  pub like_count: i32,
  pub is_paid: bool,
  pub is_premium: bool,
  pub is_family_friendly: bool,
  pub allowed_regions: Vec<String>,
  pub genre: String,
  pub author: String,
  pub author_id: String,
  pub author_thumbnails: Vec<AuthorThumbnail>,
  pub sub_count_text: String,
  pub length_seconds: i32,
  pub allow_ratings: bool,
  pub is_listed: bool,
  pub is_live: bool,
  pub is_upcoming: bool,
  pub dash_url: String,
  pub adaptive_formats: Vec<AdaptiveFormat>,
  pub format_streams: Vec<FormatStream>,
  pub captions: Vec<Caption>,
  pub recommended_videos: Vec<RecommendedVideo>,
  pub client_context: ClientContext
}

impl Video {
  pub fn from_json_responses(json_object : Value) -> Video {
    let title = match json_object["player"]["videoDetails"]["title"].as_str() {
      Some(title_string) => String::from(title_string),
      None => String::from("")
    };
    let video_id = match json_object["player"]["videoDetails"]["videoId"].as_str() {
      Some(video_id_string) => String::from(video_id_string),
      None => String::from("")
    };
    // The thumbnails that come from the player_response are a fraction of the publically available thumbnails,
    // so I just generate links to them by hand here:
    let thumbnails = generate_yt_video_thumbnails(&video_id);
    let description = String::from(match json_object["player"]["videoDetails"]["shortDescription"].as_str() {
      Some(description_string) => description_string,
      None => ""
    });
    // I would be able to remove the need for mutables here if I knew who to do something like `.Where`
    let mut video_primary_renderer_index = 0;
    let mut video_secondary_renderer_index = 1;
    for i in 0..3 {
      // 🔎Check things that should exist inside of videoSecondaryInfoRenderer in order to find which element contains it
      match json_object["next"]["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][i]["videoPrimaryInfoRenderer"]["trackingParams"].as_str() {
        // ^^🤔For some reason checking for videoPrimaryInfoRenderer directly results in the `Some` path running every time
        Some(_) => {
          video_primary_renderer_index = i
        },
        None => {}
      }
      // 🔎Check things that should exist inside of videoSecondaryInfoRenderer in order to find which element contains it
      match json_object["next"]["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][i]["videoSecondaryInfoRenderer"]["descriptionCollapsedLines"].as_i64() {
        // ^^🤔For some reason checking for videoSecondaryInfoRenderer directly results in the `Some` path running every time
        Some(_) => {
          video_secondary_renderer_index = i
        },
        None => {}
      }
    }
    // 📕 Parse out the description html from the next results 
    let mut description_html = String::from("");
    match json_object["next"]["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][video_secondary_renderer_index]["videoSecondaryInfoRenderer"]["description"].as_object() {
      Some(_description_object) => {
        let description_object = &json_object["next"]["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][video_secondary_renderer_index]["videoSecondaryInfoRenderer"]["description"];
        match description_object["runs"].as_array() {
          Some(runs) => {
            // don't iterate or access runs directly because it will panic if a key doesn't exist!!!!
            for i in 0..runs.len() {
              match description_object["runs"][i]["text"].as_str() {
                Some(text) => {
                  match description_object["runs"][i]["navigationEndpoint"]["urlEndpoint"].as_object() {
                    Some(url_endpoint) => {
                      // is link
                      match url_endpoint["url"].as_str() {
                        Some(url_string) => {
                          match Regex::new(r"&q=([^&]*?)&").unwrap().captures(url_string) {
                            Some(captures) => {
                              match captures.get(1) {
                                Some(capture) => {
                                  description_html = format!("{}<a href=\"{}\">{}</a>", description_html,urldecode::decode(String::from(capture.as_str())), text);
                                },
                                None => {}
                              }
                            },
                            None => {}
                          }
                        },
                        None => {}
                      }
                    },
                    None => {
                      // Handle cases where the link is relative to the website URL
                      match description_object["runs"][i]["navigationEndpoint"]["commandMetadata"]["webCommandMetadata"]["url"].as_str() {
                        Some(url) => {
                          description_html = format!("{}<a href=\"{}{}\">{}</a>", description_html, WEBSITE_BASE_URL, urldecode::decode(String::from(url)), text);
                        },
                        None => {
                          // Handle cases where the link actually just contains a video id
                          match description_object["runs"][i]["navigationEndpoint"]["watchEndpoint"]["videoId"].as_str() {
                            Some(video_id) => {
                              description_html = format!("{}<a href=\"{}\">{}</a>", description_html,format!("https://youtube.com/watch?v={}", video_id), text);
                            },
                            None => {
                              description_html = format!("{}{}", description_html, text);
                            }
                          }
                        }
                      }

                    }
                  }
                },
                None => {}
              }
            }
          },
          None => {}
        }
      },
      None => {}
    }
    let published_text = String::from(match json_object["next"]["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][video_primary_renderer_index]["videoPrimaryInfoRenderer"]["relativeDateText"]["simpleText"].as_str() {
      Some(relative_date_text) => relative_date_text,
      None => {
        match json_object["next"]["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][video_primary_renderer_index]["videoPrimaryInfoRenderer"]["dateText"]["simpleText"].as_str() {
          Some(date_text) => date_text,
          None => ""
        }
      }
    });
    let published_date_string = match json_object["next"]["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][video_primary_renderer_index]["videoPrimaryInfoRenderer"]["dateText"]["simpleText"].as_str() {
      Some(date_text) => date_text.replace(",","").replace(" ", "-").to_lowercase().replace("streamed-live-on-", "").replace("started-streaming-on-", ""),
      None => String::from("")
    };
    let published_timestamp : i64 = match NaiveDate::parse_from_str(&published_date_string, "%b-%d-%Y") {
      Ok(date_time) => {
        date_time.and_time(NaiveTime::default()).timestamp()
      },
      Err(error) => {
        warn!("{}", error);
        0
      }
    };
    let keywords : Vec::<String> = match json_object["player"]["videoDetails"]["keywords"].as_array() {
      Some(keywords_array) => {
        keywords_array.iter().map(|keyword| {
          String::from(match keyword.as_str() {
            Some(keyword_string) => keyword_string,
            None => ""
          })
        }).collect::<Vec<String>>()
      },
      None => Vec::<String>::new()
    };
    let view_count : i64 = match json_object["player"]["videoDetails"]["viewCount"].as_str() {
      Some(view_count) => {
        match view_count.parse::<i64>() {
          Ok(i_view_count) => i_view_count,
          Err(_err) => 0
        }
      },
      None => 0
    };
    let like_count = match json_object["next"]["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][0]["videoPrimaryInfoRenderer"]["videoActions"]["menuRenderer"]["topLevelButtons"][0]["segmentedLikeDislikeButtonRenderer"]["likeButton"]["toggleButtonRenderer"]["toggledText"]["accessibility"]["accessibilityData"]["label"].as_str() {
      Some(string_value) => {
        match Regex::new(r"([0-9]*?) likes").unwrap().captures(&string_value.replace(",", "")) {
          Some(groups) => {
            if groups.len() > 1 {
              match groups[1].parse::<i32>() {
                Ok(i_value) => i_value,
                Err(_err) => 0
              }
            } else {
              0
            }
          },
          None => 0
        }
      },
      None => {
        match json_object["next"]["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][0]["videoPrimaryInfoRenderer"]["videoActions"]["menuRenderer"]["topLevelButtons"][0]["toggleButtonRenderer"]["defaultText"]["accessibility"]["accessibilityData"]["label"].as_str() {
          Some(likes_label) => {
            match Regex::new(r"([0-9,]*?) likes").unwrap().captures(&likes_label) {
              Some(likes_captures) => {
                match likes_captures[1].replace(",", "").parse::<i32>() {
                  Ok(likes) => likes,
                  Err(_) => 0
                }
              },
              None => 0
            }
          },
          None => {
            0
          }
        }
      }
    };
    let is_family_safe = match json_object["player"]["microformat"]["playerMicroformatRenderer"]["isFamilySafe"].as_bool() {
      Some(is_family_safe) => is_family_safe,
      None => false
    };
    let allowed_regions = match json_object["player"]["microformat"]["playerMicroformatRenderer"]["availableCountries"].as_array() {
      Some(regions) => {
        regions.iter().map(|region| {
          String::from(match region.as_str() {
            Some(region) => region,
            None => ""
          })
        }).collect::<Vec<String>>()
      },
      None => Vec::<String>::new()
    };
    let genre = String::from(match json_object["player"]["microformat"]["playerMicroformatRenderer"]["category"].as_str() {
      Some(category) => category,
      None => ""
    });
    let author = String::from(match json_object["player"]["videoDetails"]["author"].as_str() {
      Some(author) => author,
      None => ""
    });
    let author_id = String::from(match json_object["player"]["videoDetails"]["channelId"].as_str() {
      Some(author_id) => author_id,
      None => ""
    });
    let author_thumbnails = match json_object["next"]["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][video_secondary_renderer_index]["videoSecondaryInfoRenderer"]["owner"]["videoOwnerRenderer"]["thumbnail"]["thumbnails"].as_array() {
      Some(thumbnails) => {
        thumbnails.iter().map(|thumbnail| {
          let url = match thumbnail["url"].as_str() {
            Some(url) => url,
            None => ""
          };
          let width = match thumbnail["width"].as_i64() {
            Some(width) => width,
            None => 0
          };
          let height = match thumbnail["height"].as_i64() {
            Some(height) => height,
            None => 0
          };
          AuthorThumbnail { url: String::from(url), width: width as i32, height: height as i32 }
        }).collect::<Vec<AuthorThumbnail>>()
      },
      None => Vec::<AuthorThumbnail>::new()
    };
    let sub_count = match json_object["next"]["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][video_secondary_renderer_index]["videoSecondaryInfoRenderer"]["owner"]["videoOwnerRenderer"]["subscriberCountText"]["accessibility"]["accessibilityData"]["label"].as_str() {
      Some(sub_count) => {
        match Regex::new("([0-9.]*? *[a-zA-Z]*?) subscribers").unwrap().captures(sub_count) {
          Some(captures) => {
            captures[1].replace(" million", "M")
          },
          None => String::from("")
        }
      },
      None => String::from("")
    };
    let length_seconds = match json_object["player"]["videoDetails"]["lengthSeconds"].as_str() {
      Some(length) => {
        match length.parse::<i64>() {
          Ok(i_length) => i_length,
          Err(_err) => 0
        }
      },
      None => 0
    };
    let allow_ratings = match json_object["player"]["videoDetails"]["allowRatings"].as_bool() {
      Some(allow_ratings) => allow_ratings,
      None => false
    };
    let is_unlisted = match json_object["player"]["videoDetails"]["isUnlisted"].as_bool() {
      Some(is_unlisted) => is_unlisted,
      None => false
    };
    let is_live = match json_object["player"]["responseContext"]["serviceTrackingParams"][0]["params"].as_array() {
      Some(tracking_params) => {
        //😣i feel like i shouldn't need a mutable to accomplish this task
        let mut is_live = false;
        for i in 0..tracking_params.len() {
          is_live = is_live || match json_object["player"]["responseContext"]["serviceTrackingParams"][0]["params"][i]["key"].as_str() {
            Some(key) => {
              if key == "is_viewed_live" {
                match  json_object["player"]["responseContext"]["serviceTrackingParams"][0]["params"][i]["value"].as_str() {
                  Some(value) => value == "True",
                  None => false
                }
              } else {
                false
              }
            },
            None => false
          };
        }
        is_live
      },
      None => false
    };
    // This only works for some videos and livestreams
    // TODO : figure out where I can pull this data from for some edge cases
    let dash_manifest_url = String::from(match json_object["player"]["streamingData"]["dashManifestUrl"].as_str() {
      Some(dash_manifest_url) => dash_manifest_url,
      None => ""
    });
    let adaptive_formats = match json_object["player"]["streamingData"]["adaptiveFormats"].as_array() {
      Some(formats) => {
        formats.iter().map(|format| {
          let init_start = match format["initRange"]["start"].as_str() {
            Some(start) => match start.parse::<i32>() {
              Ok(start_i) => start_i,
              Err(error) => {
                warn!("{}", error);
                0
              }
            },
            None => 0
          };
          let init_end = match format["initRange"]["end"].as_str() {
            Some(end) => match end.parse::<i32>() {
              Ok(end_i) => end_i,
              Err(error) => {
                warn!("{}", error);
                0
              }
            },
            None => 0
          };
          let index_start = match format["indexRange"]["start"].as_str() {
            Some(start) => match start.parse::<i32>() {
              Ok(start_i) => start_i,
              Err(error) => {
                warn!("{}", error);
                0
              }
            },
            None => 0
          };
          let index_end = match format["indexRange"]["end"].as_str() {
            Some(end) => match end.parse::<i32>() {
              Ok(end_i) => end_i,
              Err(error) => {
                warn!("{}", error);
                0
              }
            },
            None => 0
          };
          let bitrate = match format["bitrate"].as_i64() {
            Some(bitrate) => bitrate as i32,
            None => 0
          };
          let url =  match format["url"].as_str() {
            Some(url) => String::from(url),
            None => match format["signatureCipher"].as_str() {
              Some(cipher) => {
                match Regex::new(r"&url=(.*)").unwrap().captures(cipher) {
                  Some(captures) => {
                    if captures.len() > 1 {
                      // TODO : actually decipher the decoded url
                      let decoded_url = urldecode::decode(String::from(&captures[1]));
                      cipher.to_string()
                    } else {
                      String::from("")
                    }
                  },
                  None => String::from("")
                }
              },
              None => String::from("")
            }
          };
          let itag = match format["itag"].as_i64() {
            Some(itag) => itag as i32,
            None => 0
          };
          let mime_type = String::from(match format["mimeType"].as_str() {
            Some(mime_type) => mime_type,
            None => ""
          });
          let content_length = match format["contentLength"].as_str() {
            Some(content_length) => {
              match content_length.parse::<i32>() {
                Ok(i_content_length) => i_content_length,
                Err(err) => {
                  warn!("{}", err);
                  0
                }
              }
            },
            None => 0
          };
          let last_modified_time = match format["lastModified"].as_str() {
            Some(last_modified_time) => {
              match last_modified_time.parse::<i64>() {
                Ok(i_last_modified_time) => i_last_modified_time,
                Err(err) => {
                  warn!("{}", err);
                  0
                }
              }
            },
            None => 0
          };
          let projection_type = String::from(match format["projectionType"].as_str() {
            Some(projection_type) => projection_type,
            None => ""
          });
          let fps = match format["fps"].as_i64() {
            Some(fps) => Some(fps as i32),
            None => None
          };
          let quality_label = match format["qualityLabel"].as_str() {
            Some(quality_label) => Some(String::from(quality_label)),
            None => None
          };
          let audio_quality = match format["audioQuality"].as_str() {
            Some(audio_quality) => Some(String::from(audio_quality)),
            None => None
          };
          let audio_sample_rate = match format["audioSampleRate"].as_str() {
            Some(audio_sample_rate) => {
              match audio_sample_rate.parse::<i32>() {
                Ok(audio_sample) => Some(audio_sample),
                Err(error) => {
                  warn!("{}", error);
                  None
                }
              }
            },
            None => None
          };
          let audio_channels = match format["audioChannels"].as_i64() {
            Some(audio_channels) => Some(audio_channels as i32),
            None => None
          };
          let container = match Regex::new(r"/([a-zA-Z0-9.]*?);").unwrap().captures(&mime_type) {
            Some(captures) => {
              if captures.len() > 1 {
                String::from(&captures[1])
              } else {
                String::from("")
              }
            },
            None => String::from("")
          };
          let encoding = match Regex::new(r#"codecs="([a-zA-Z0-9.]*?)""#).unwrap().captures(&mime_type) {
            Some(captures) => {
              if captures.len() > 1 {
                String::from(&captures[1])
              } else {
                String::from("")
              }
            },
            None => String::from("")
          };
          AdaptiveFormat {
            init: Range {
              start: init_start,
              end: init_end
            },
            index: Range {
              start: index_start,
              end: index_end
            },
            bitrate: bitrate,
            url: url,
            itag: itag,
            mime_type: mime_type,
            content_length: content_length,
            last_modified_time: last_modified_time,
            projection_type: projection_type,
            fps: fps,
            quality_label: quality_label,
            audio_quality: audio_quality,
            audio_sample_rate: audio_sample_rate,
            audio_channels: audio_channels,
            container: container,
            encoding: encoding
          }
        }).collect::<Vec<AdaptiveFormat>>()
      },
      None => Vec::<AdaptiveFormat>::new()
    };
    let format_streams = match json_object["player"]["streamingData"]["formats"].as_array() {
      Some(format_streams) => {
        format_streams.iter().map(|format| {
          let url = match format["url"].as_str() {
            Some(url) => String::from(url),
            None => match format["signatureCipher"].as_str() {
              Some(cipher) => {
                match Regex::new(r"&url=(.*)").unwrap().captures(cipher) {
                  Some(captures) => {
                    if captures.len() > 1 {
                      // TODO : actually decipher the decoded url
                      let decoded_url = urldecode::decode(String::from(&captures[1]));
                      cipher.to_string()
                    } else {
                      String::from("")
                    }
                  },
                  None => String::from("")
                }
              },
              None => String::from("")
            }
          };
          let itag = match format["itag"].as_i64() {
            Some(itag) => itag as i32,
            None => 0
          };
          let mime_type = String::from(match format["mimeType"].as_str() {
            Some(mime_type) => mime_type,
            None => ""
          });
          let quality = String::from(match format["quality"].as_str() {
            Some(quality) => quality,
            None => ""
          });
          let quality_label = String::from(match format["qualityLabel"].as_str() {
            Some(quality_label) => quality_label,
            None => ""
          });
          let fps = match format["fps"].as_i64() {
            Some(fps) => fps as i32,
            None => 0
          };
          let container = match Regex::new(r"/([a-zA-Z0-9.]*?);").unwrap().captures(&mime_type) {
            Some(captures) => {
              if captures.len() > 1 {
                String::from(&captures[1])
              } else {
                String::from("")
              }
            },
            None => String::from("")
          };
          let encoding = match Regex::new(r#"codecs="([a-zA-Z0-9.]*?)""#).unwrap().captures(&mime_type) {
            Some(captures) => {
              if captures.len() > 1 {
                String::from(&captures[1])
              } else {
                String::from("")
              }
            },
            None => String::from("")
          };
          let width = match format["width"].as_i64() {
            Some(width) => width as i32,
            None => 0
          };
          let height = match format["height"].as_i64() {
            Some(height) => height as i32,
            None => 0
          };
          FormatStream {
            url: url,
            itag: itag,
            mime_type: mime_type,
            quality: quality,
            fps: fps,
            container: container,
            encoding: encoding,
            quality_label: quality_label,
            size: Size::new(width, height)
          }
        }).collect::<Vec<FormatStream>>()
      },
      None => Vec::new()
    };
    let captions = match json_object["player"]["captions"]["playerCaptionsTracklistRenderer"]["captionTracks"].as_array() {
      Some(caption_tracks) => {
        caption_tracks.iter().map(|caption_track| {
          let url = match caption_track["baseUrl"].as_str() {
            Some(url) => {
              // the base url scraped from the initial page data is in a weird format
              // this makes it vtt
              url.replace("&ip=0.0.0.0", "&ip=0.0.0.0&fmt=vtt")
            }
            None => String::from("")
          };
          let label = String::from(match caption_track["name"]["simpleText"].as_str() {
            Some(label) => label,
            None => ""
          });
          let language_code = String::from(match caption_track["languageCode"].as_str() {
            Some(language_code) => language_code,
            None => ""
          });
          Caption {
            url: url,
            label: label,
            language_code: language_code
          }
        }).collect::<Vec<Caption>>()
      },
      None => Vec::new()
    };
    let client_context = ClientContext::from_json(&json_object["config"]);
    let recommended_videos = match json_object["next"]["contents"]["twoColumnWatchNextResults"]["secondaryResults"]["secondaryResults"]["results"].as_array() {
      Some(recommended_videos) => {
        recommended_videos.iter().filter_map(|recommended_video| {
          // only show recommended videos with video ids
          match recommended_video["compactVideoRenderer"]["videoId"].as_str() {
            Some(_) => {
              Some(RecommendedVideo::from_compact_video_renderer(&recommended_video["compactVideoRenderer"]))
            },
            None => None
          }
        }).collect::<Vec<RecommendedVideo>>()
      },
      None => Vec::new()
    };
    Video { 
      title: title,
      video_id: video_id,
      video_thumbnails: thumbnails,
      // TODO: finish pulling storyboards
      // UPDATE : I found the implementation for this inside of node-ytdl-core
      storyboards: Vec::<Value>::new(),
      description: description,
      description_html: description_html.replace("\n", "<br/>"),
      published: published_timestamp as i32,
      published_text: published_text,
      keywords: keywords,
      
      view_count: view_count as i32,
      like_count: like_count as i32,
      
      // I don't have any way to know these right now
      is_paid: false,
      is_premium: false,

      is_family_friendly: is_family_safe,
      allowed_regions: allowed_regions,
      genre: genre,
      author: author,
      author_id: author_id,
      author_thumbnails: author_thumbnails,
      sub_count_text: sub_count,
      length_seconds: length_seconds as i32,
      allow_ratings: allow_ratings,
      is_listed: !is_unlisted,
      is_live: is_live,
      is_upcoming: false,
      dash_url: dash_manifest_url,
      adaptive_formats: adaptive_formats,
      format_streams: format_streams,
      captions: captions,
      recommended_videos: recommended_videos,
      client_context: client_context
    }
  }
  // Formats the video object into the same format that the invidious API would return
  pub fn fmt_inv(self) -> Result<String, serde_json::Error> {
    // I feel like I could easily accomplish this same task without a mutable
    // by creating a private struct with these camel case properties
    // however, one benefit of this method is fully optional json properties in the output
    // which makes this output more similar to the output of invidious
    let mut j_object = serde_json::Map::new();
    j_object.insert(String::from("type"), Value::String(String::from("video")));
    j_object.insert(String::from("title"), Value::String(String::from(self.title)));
    j_object.insert(String::from("videoId"), Value::String(String::from(self.video_id)));
    j_object.insert(String::from("videoThumbnails"), self.video_thumbnails.iter().map(|thumbnail| {
      json!(thumbnail)
    }).collect());
    j_object.insert(String::from("storyboards"), Value::Array(Vec::<Value>::new()));
    j_object.insert(String::from("description"), Value::String(self.description.replace("\n", "\\n")));
    j_object.insert(String::from("descriptionHtml"), Value::String(self.description_html));
    j_object.insert(String::from("published"), json!(self.published));
    j_object.insert(String::from("publishedText"), json!(self.published_text));
    j_object.insert(String::from("keywords"), json!(self.keywords));
    j_object.insert(String::from("viewCount"), json!(self.view_count));
    j_object.insert(String::from("likeCount"), json!(self.like_count));

    // [ IMPORTANT NOTE: ]
    // YT doesn't provide this anymore🤷
    // I am _not_ going to pull this data from the "Return Youtube Dislike" service
    // for the exact same reason that this service will not be used for FreeTube:
    // it isn't fully open source and it can't be ran by a neutral 3rd party
    // See : https://github.com/FreeTubeApp/FreeTube/issues/1927#issuecomment-1007121970 for full explanation
    j_object.insert(String::from("dislikeCount"), json!(0));

    j_object.insert(String::from("paid"), json!(self.is_paid));
    j_object.insert(String::from("premium"), json!(self.is_premium));
    j_object.insert(String::from("isFamilyFriendly"), json!(self.is_family_friendly));
    j_object.insert(String::from("allowedRegions"), json!(self.allowed_regions));
    j_object.insert(String::from("genre"), json!(self.genre));

    // 🤔 I don't know if this exists anymore because 
    // I can't figure out what the category pages are supposed to be.
    // Invidious always returns the string "/channel/" for this value, so
    // for now, that is what I am doing here.
    j_object.insert(String::from("genreUrl"), json!("/channel/"));

    j_object.insert(String::from("author"), json!(self.author));
    j_object.insert(String::from("authorId"), json!(self.author_id));
    j_object.insert(String::from("authorUrl"), json!(format!("/channel/{}", self.author_id)));
    j_object.insert(String::from("authorThumbnails"), json!(self.author_thumbnails));
    j_object.insert(String::from("subCountText"), json!(self.sub_count_text));
    j_object.insert(String::from("lengthSeconds"), json!(self.length_seconds));
    j_object.insert(String::from("allowRatings"), json!(self.allow_ratings));

    // YT also doesn't provide this anymore🤷
    // It is wrapped into the same reason that they don't provide dislikes.
    // Back when this was provided, dislikes could be calculated despite the fact
    // that dislikes are typically hidden.
    j_object.insert(String::from("rating"), json!(0));

    j_object.insert(String::from("isListed"), json!(self.is_listed));
    j_object.insert(String::from("liveNow"), json!(self.is_live));
    j_object.insert(String::from("dashUrl"), json!(self.dash_url));
    j_object.insert(String::from("adaptiveFormats"), json!(self.adaptive_formats.iter().map(|format| {
      // 😞I feel like I shouldn't need a clone here, but idk
      format.clone().fmt_inv()
    }).collect::<Vec<serde_json::Map<std::string::String, serde_json::Value>>>()));
    j_object.insert(String::from("formatStreams"), json!(self.format_streams.iter().map(|format| {
      // 😞I feel like I shouldn't need a clone here, but idk
      format.clone().fmt_inv()
    }).collect::<Vec<serde_json::Map<std::string::String, serde_json::Value>>>()));
    j_object.insert(String::from("captions"), json!(self.captions.iter().map(|caption| {
      serde_json::from_str::<Value>(&serde_json::to_string_pretty(&caption).unwrap()).unwrap()
    }).collect::<Vec<Value>>()));
    j_object.insert(String::from("recommendedVideos"), json!(self.recommended_videos.iter().map(|recommended_video| {
      recommended_video.clone().fmt_inv()
    }).collect::<Vec<serde_json::Map<std::string::String, serde_json::Value>>>()));
    Ok(serde_json::to_string_pretty(&j_object).unwrap())
  }
}
