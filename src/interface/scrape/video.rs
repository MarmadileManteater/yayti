
use serde_json::Value;

use reqwest::Client;
use regex::Regex;
use log::warn;

use super::super::super::constants::{WEBSITE_BASE_URL, WEBSITE_VIDEO_PATH};

#[derive(Debug)]
pub struct ScrapeVideoError {
  pub message: String,
  pub inner_reqwest_error: Option<reqwest::Error>,
  pub inner_serde_error: Option<serde_json::Error>
}
impl ScrapeVideoError {
  fn new(message: &str) -> ScrapeVideoError {
    ScrapeVideoError { message: String::from(message), inner_reqwest_error: None, inner_serde_error: None }
  }
  fn new_reqwest(message:  &str, inner_reqwest_error : reqwest::Error) -> ScrapeVideoError {
    ScrapeVideoError { message: String::from(message), inner_reqwest_error: Some(inner_reqwest_error), inner_serde_error: None }
  }
  fn new_serde(message: &str, inner_serde_error : serde_json::Error) -> ScrapeVideoError {
    ScrapeVideoError { message: String::from(message), inner_reqwest_error: None, inner_serde_error: Some(inner_serde_error) }
  }
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

// 📄Creates a URL given a base, id, and an optional language (defaults to `en`)
fn create_page_url(base_url : &str, id : &str, lang : Option<&str>) -> String {
  if lang == None {
    // Language defaults to EN
    return format!("{}{}&hl=en", base_url, id)
  }
  format!("{}{}&hl={}", base_url, id, lang.unwrap())
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

// 📡Retrieves the video info from YT by manually scraping the video page
pub async fn get_info(id : &str, lang : Option<&str>) -> Result<Value, ScrapeVideoError> {
  match get_page_body(&format!("{}{}", WEBSITE_BASE_URL, WEBSITE_VIDEO_PATH), id, lang).await {
    Ok(page_body) => {
      // Pull the initial player response from using the 1st group matched in this regular expression
      let initial_player_response = match find_json(&page_body, Regex::new(r"\bytInitialPlayerResponse\s*=\s*(\{.*?);</script").unwrap(), 1) {
        Ok(initial_player_response) => initial_player_response,
        Err(error) => {
          warn!("{}", error.message);
          String::from("{}")
        }
      };
      // Pull the initial next response from using the 2nd group matched in this regular expression
      let initial_next_response = match find_json(&page_body, Regex::new(r#"\bytInitialData("\])?\s*=\s*(\{.*?);</script"#).unwrap(), 2) {
        Ok(initial_next_response) => initial_next_response,
        Err(error) => {
          warn!("{}", error.message);
          String::from("{}")
        }
      };
      // Pull the API config data from the page body
      let api_config = match find_json(&page_body, Regex::new(r#"\(function\(\) \{window\.ytplayer=\{\};[\r\n]*ytcfg\.set\((\{.*?\})\); window\.ytcfg\.obfuscatedData_ ="#).unwrap(), 1) {
        Ok(yt_config) => {
          yt_config
        },
        Err(error) => {
          warn!("{}", error.message);
          String::from("")
        }
      };
      // Format all of the json pulled into an expected JSON object
      match serde_json::from_str::<Value>(&format!("{} \"player\":{}, \"next\":{}, \"config\": {} {}", "{", initial_player_response, initial_next_response, api_config, "}")) {
        Ok(player_response) => {
          Ok(player_response)
        },
        Err(error) => Err(ScrapeVideoError::new_serde("Failed to parse initial player response", error))
      }
    },
    Err(error) => return Err(ScrapeVideoError::new_reqwest("Error when making HTTP(s) request", error))
  }
}
