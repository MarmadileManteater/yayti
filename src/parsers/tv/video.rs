
use serde_json::Value;
use crate::constants::{SHORT_WEBSITE_BASE_URL};

// Gets the title from the `next` endpoint response or the `player` endpoint response in tv responses
pub fn get_title(json: &Value) -> Option<String> {
match json["contents"]["singleColumnWatchNextResults"]["results"]["results"]["contents"][0]["itemSectionRenderer"]["contents"][0]["videoMetadataRenderer"]["title"]["runs"][0]["text"].as_str() {
    Some(title) => Some(String::from(title)),
    None => None
  }
}

// Gets the description from the `next` endpoint response or the `player` endpoint response in android
pub fn get_description(json: &Value) -> Option<String> {
  match json["contents"]["singleColumnWatchNextResults"]["results"]["results"]["contents"][0]["itemSectionRenderer"]["contents"][0]["videoMetadataRenderer"]["description"]["runs"].as_array() {
    Some(runs) => {
      Some(runs.into_iter().map(|run| {
        match run["text"].as_str() {
          Some(text) => text,
          None => ""
        }
      }).collect::<String>())
    },
    None => None
  }
}

// Gets the description html from the `next` endpoint response 
pub fn get_description_html(json: &Value) -> Option<String> {
  match json["contents"]["singleColumnWatchNextResults"]["results"]["results"]["contents"][0]["itemSectionRenderer"]["contents"][0]["videoMetadataRenderer"]["description"]["runs"].as_array() {
      Some(runs) => {
        Some(runs.into_iter().map(|run| {
          let url = match run["navigationEndpoint"]["watchEndpoint"]["videoId"].as_str() {
            Some(video_id) => {
              match run["navigationEndpoint"]["watchEndpoint"]["startTimeSeconds"].as_i64() {
                Some(start_time_seconds) if start_time_seconds > 0 => Some(format!("{}/{}?t={}", SHORT_WEBSITE_BASE_URL, video_id, start_time_seconds)),
                Some(_start_time_seconds) => Some(format!("{}/{}", SHORT_WEBSITE_BASE_URL, video_id)),
                None => {
                  Some(format!("{}/{}", SHORT_WEBSITE_BASE_URL, video_id))
                }
              }
            },
            None => match run["navigationEndpoint"]["urlEndpoint"]["url"].as_str() {
              Some(url) => {
                Some(String::from(url))
              },
              None => None
            }
          };
          let text = match run["text"].as_str() {
            Some(text) => Some(String::from(text)),
            None => None
          };
          match url {
            Some(url) => {
              match text {
                Some(text) => format!("<a href=\"{}\">{}</a>", url, text),
                None => format!("<a href=\"{}\">{}</a>", url, url)
              }
            },
            None => match text {
              Some(text) => text.to_string(),
              None => "".to_string()
            }
          }
        }).collect::<String>())
      },
      None => None
    }
}
