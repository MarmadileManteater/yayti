
use crate::constants::WEBSITE_BASE_URL;
use regex::Regex;

pub async fn get_player_js_id() -> Result<String,Option<reqwest::Error>> {
  let client = reqwest::Client::new();
  match client.get(format!("{}/iframe_api", WEBSITE_BASE_URL)).send().await {
    Ok(response) => {
      match response.text().await {
        Ok(response_text) => {
          let player_url_regex =Regex::new(r"= 'https:\\/\\/www.youtube.com\\/s\\/player\\/([^']*?)\\/www-widgetapi.vflset\\/www-widgetapi.js';window\['yt_embeds").unwrap();
          match player_url_regex.captures(&response_text) {  
            Some(captures) => {
              Ok(format!("{}", captures.get(1).unwrap().as_str()))
            },
            None => {
              Err(None)
            }
          }
        },
        Err(err) => Err(Some(err))
      }
    },
    Err(err) => Err(Some(err))
  }
}

pub async fn get_player_response(id: &str) -> Result<String,reqwest::Error> {
  let url = format!("{}/s/player/{}/player_ias.vflset/en_US/base.js", WEBSITE_BASE_URL, id);
  let client = reqwest::Client::new();
  match client.get(url).send().await {
    Ok(response) => {
      match response.text().await {
        Ok(response_text) => Ok(response_text),
        Err(error) => Err(error)
      }
    },
    Err(error) => Err(error)
  }
}
