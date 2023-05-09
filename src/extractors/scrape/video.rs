
use serde_json::Value;

use reqwest::Client;
use regex::Regex;
use log::warn;

use crate::constants::{WEBSITE_BASE_URL, WEBSITE_VIDEO_PATH};

#[derive(Debug)]
pub struct ScrapeVideoError {
  pub message: String,
  pub inner_reqwest_error: Option<reqwest::Error>
}

impl ScrapeVideoError {
  fn new(message: &str) -> ScrapeVideoError {
    ScrapeVideoError { message: String::from(message), inner_reqwest_error: None }
  }
  fn new_reqwest(message:  &str, inner_reqwest_error : reqwest::Error) -> ScrapeVideoError {
    ScrapeVideoError { message: String::from(message), inner_reqwest_error: Some(inner_reqwest_error) }
  }
}

// 📄Creates a URL given a base, id, and an optional language (defaults to `en`)
fn create_page_url(base_url : &str, id : &str, lang : Option<&str>) -> String {
  if lang == None {
    // Language defaults to EN
    return format!("{}{}&hl=en", base_url, id)
  }
  format!("{}{}&hl={}", base_url, id, lang.unwrap())
}

// 🔍Finds a JSON string inside of a given page body
fn find_json(body : &str, pattern : Regex, group_index : usize) -> Result<String,ScrapeVideoError> {
  match pattern.captures(body) {
    Some(result) => {
      match result.get(group_index) {
        Some (group) => {
          Ok(format!("{}", group.as_str() ))
        },
        None => Err(ScrapeVideoError::new("Unable to find JSON"))
      }
    },
    None => Err(ScrapeVideoError::new("Unable to find JSON"))
  }
}

// 📡Retrieves the body content of the page given the base_url, the video id, and the langauge preference
async fn get_page_body(base_url : & str, id : &str, lang : Option<&str>) -> Result<String, reqwest::Error> {
  let client = Client::new();
  let url = create_page_url(base_url, id, lang);
  match client.get(url).send().await {
    Ok(response) => {
      match response.text().await {
        Ok(string_response) => return Ok(string_response),
        Err(error) => return Err(error)
      }
    },
    Err(error) => return Err(error)
  }
}

pub struct VideoPageResponse {
  pub next: String,
  pub player: String,
  pub config: String
}

pub async fn scrape_video_page(video_id: &str, lang: Option<&str>) -> Result<VideoPageResponse, Vec::<ScrapeVideoError>> {
  match get_page_body(&format!("{}{}", WEBSITE_BASE_URL, WEBSITE_VIDEO_PATH), video_id, lang).await {
    Ok(page_body) => {
      let mut errors = vec![];

      // Pull the initial player response from using the 1st group matched in this regular expression
      let initial_player_response = find_json(&page_body, Regex::new(r"\bytInitialPlayerResponse\s*=\s*(\{.*?);</script").unwrap(), 1).map_err(|e| errors.push(e)).unwrap_or(String::from(""));
      // Pull the initial next response from using the 2nd group matched in this regular expression
      let initial_next_response = find_json(&page_body, Regex::new(r#"\bytInitialData("\])?\s*=\s*(\{.*?);</script"#).unwrap(), 2).map_err(|e| errors.push(e)).unwrap_or(String::from(""));
      // Pull the API config data from the page body
      let api_config = find_json(&page_body, Regex::new(r#"\(function\(\) \{window\.ytplayer=\{\};[\r\n]*ytcfg\.set\((\{.*?\})\); window\.ytcfg\.obfuscatedData_ ="#).unwrap(), 1).map_err(|e| errors.push(e)).unwrap_or(String::from(""));
      if errors.len() == 0 {
        Ok(VideoPageResponse {
          next: initial_next_response,
          player: initial_player_response,
          config: api_config
        })
      } else {
        Err(errors)
      }
    },
    Err(error) => Err(vec!(ScrapeVideoError::new_reqwest("", error)))
  }
}
