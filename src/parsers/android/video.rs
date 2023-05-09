
use serde_json::Value;
use std::str::FromStr;

// Gets the title from the `next` endpoint response or the `player` endpoint response in android
pub fn get_title(json: &Value) -> Option<String> {
  match json["contents"]["singleColumnWatchNextResults"]["results"]["results"]["contents"][0]["slimVideoMetadataSectionRenderer"]["contents"][0]["elementRenderer"]["newElement"]["type"]["componentType"]["model"]["videoMetadataModel"]["videoMetadata"]["title"]["content"].as_str() {
    Some(title) => Some(String::from(title)),
    None => match json["videoDetails"]["title"].as_str() {
      Some(title) => Some(String::from(title)),
      None => None
    }
  }
}

// Gets the description from the `player` endpoint response in android
pub fn get_description(json: &Value) -> Option<String> {
  match json["videoDetails"]["shortDescription"].as_str() {
    Some(description) => Some(String::from(description)),
    None => None
  }
}

// Gets the playability from the `player` endpoint response in android
pub fn get_playability(json: &Value) -> bool {
  match json["playabilityStatus"]["status"].as_str() {
    Some(status) => status != "ERROR",
    None => false
  }
}

pub fn get_keywords(json: &Value) -> Option<Vec::<String>> {
  match json["videoDetails"]["keywords"].as_array() {
    Some(keyword_array) => Some(keyword_array.into_iter().filter_map(|keyword| {
      match keyword.as_str() {
        Some(keyword) => Some(String::from(keyword)),
        None => None
      }
    }).collect()),
    None => None
  }
}

pub fn get_length_seconds(json: &Value) -> Option<i32> {
  match json["videoDetails"]["lengthSeconds"].as_str() {
    Some(length_str) => {
      match i32::from_str(&length_str) {
        Ok(length) => Some(length),
        Err(_) => None
      }
    },
    None => None
  }
}

pub fn get_allow_ratings(json: &Value) -> Option<bool> {
  match json["videoDetails"]["allowRatings"].as_bool() {
    Some(rating) => Some(rating),
    None => None
  }
}
