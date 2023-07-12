
use crate::constants::{INNERTUBE_API_URL,WEBSITE_BASE_URL};
use crate::helpers::generate_player_params;
use crate::parsers::ClientContext;
use rand::Rng;

#[doc = r"Converts a serde_json map to a string to be made part of a larger JSON object"]
fn map_to_json_part(map: &serde_json::Map::<String, serde_json::Value>) -> std::string::String {
  map.into_iter().map(|(key, value) | {
    format!(",\r\n\"{}\": {}", key, value)
  }).collect::<String>()
}

pub async fn fetch_account_menu(context: &ClientContext, lang: Option<&str>) -> Result<String, reqwest::Error> {
  fetch_endpoint("account/account_menu", context, lang, serde_json::from_str::<serde_json::Map::<String, serde_json::Value>>("{}").unwrap()).await
}

#[doc = r"Fetches the `player` innertube endpoint which contains streaming data"]
pub async fn fetch_player(video_id : &str, context: &ClientContext, lang : Option<&str>) -> Result<String, reqwest::Error> {
  let random_params = generate_player_params(rand::thread_rng().gen_range(0..100)).map_err(|e| log::warn!("Problem w/ generating protobuf for player params, falling back to hardcoded params")).unwrap_or(String::from("8AEB"));
  fetch_endpoint("player", context, lang, serde_json::from_str::<serde_json::Map::<String, serde_json::Value>>(&format!(r#"
  {{
    "videoId": "{}",
    "params": "{}",
    "playbackContext": {{
      "contentPlaybackContext": {{
        "vis": 0,
        "splay": false,
        "referer": "https://www.youtube.com",
        "currentUrl": "/watch?v={}",
        "autonavState": "STATE_NONE",
        "autoCaptionsDefaultOn": false,
        "html5Preference": "HTML5_PREF_WANTS",
        "lactMilliseconds": "-1"
      }}
    }}
  }}
 "#, video_id, random_params, video_id)).unwrap()).await
}

#[doc = r"Fetches the `player` innertube endpoint which contains streaming data"]
pub async fn fetch_player_with_sig_timestamp(video_id : &str, signature_timestamp: i32, context: &ClientContext, lang : Option<&str>) -> Result<String, reqwest::Error> {
  fetch_endpoint("player", context, lang, serde_json::from_str::<serde_json::Map::<String, serde_json::Value>>(&format!(r#"
  {{
    "videoId": "{}",
    "params": "",
    "playbackContext": {{
      "contentPlaybackContext": {{
        "vis": 0,
        "splay": false,
        "referer": "https://www.youtube.com",
        "currentUrl": "/watch?v={}",
        "autonavState": "STATE_NONE",
        "signatureTimestamp": {},
        "autoCaptionsDefaultOn": false,
        "html5Preference": "HTML5_PREF_WANTS",
        "lactMilliseconds": "-1"
      }}
    }}
  }}
 "#, video_id, video_id, signature_timestamp)).unwrap()).await
}

#[doc = r"Fetches the `next` innertube endpoint which contains video data"]
pub async fn fetch_next(video_id : &str, context: &ClientContext, lang: Option<&str>) -> Result<String, reqwest::Error> {
  fetch_endpoint("next", context, lang, serde_json::from_str::<serde_json::Map::<String, serde_json::Value>>(&format!(r#"
  {{
    "videoId": "{}"
  }}
 "#, video_id)).unwrap()).await
}

#[doc = r"Fetches the `browse` innertube endpoint which contains channel or playlist data"]
pub async fn fetch_browse(id : &str, context: &ClientContext, lang: Option<&str>, extra_options : Option<serde_json::Map::<String, serde_json::Value>>) -> Result<String, reqwest::Error> {
  let extra_options_string = match extra_options {
    Some(extra_options) => {
      map_to_json_part(&extra_options)
    },
    None => "".to_string()
  };
  fetch_endpoint("browse", context, lang, serde_json::from_str::<serde_json::Map::<String, serde_json::Value>>(&format!(r#"
  {{
    "browseId": "{}"{}
  }}
 "#, id, extra_options_string)).unwrap()).await
}

#[doc = r"Fetches the `browse` innertube endpoint which contains channel or playlist data"]
pub async fn fetch_continuation(endpoint: &str, continuation: &str, context: &ClientContext, lang: Option<&str>) -> Result<String, reqwest::Error> {
  fetch_endpoint(endpoint, context, lang, serde_json::from_str::<serde_json::Map::<String, serde_json::Value>>(&format!(r#"
  {{
    "continuation": "{}"
  }}
 "#, continuation)).unwrap()).await
}

#[doc = r"Fetches the `browse` innertube endpoint which contains playlist data"]
pub async fn fetch_playlist(playlist_id: &str, context: &ClientContext, lang: Option<&str>, extra_options: Option<serde_json::Map::<String, serde_json::Value>>) -> Result<String, reqwest::Error> {
  fetch_browse(&format!("VL{}", playlist_id), context, lang, extra_options).await
}

#[doc = r"Fetches an innertube API endpoint with the given client context and language option"]
pub async fn fetch_endpoint(endpoint: &str, context: &ClientContext, lang : Option<&str>, extra_options : serde_json::Map::<String, serde_json::Value>) -> Result<String, reqwest::Error> {
  let lang = match lang {
    Some(lang) => lang,
    None => "en"
  };
  let request_string = format!(r#"
  {{
    "context": {{
      "client": {{
        "hl": "{}",
        "gl": "US",
        "remoteHost": "",
        "deviceMake": "",
        "deviceModel": "",
        "visitorData": "",
        "userAgent": "{}",
        "clientName": "{}",
        "clientVersion": "{}",
        "osName": "{}",
        "osVersion": "{}",
        "originalUrl": "",
        "platform": "{}",
        "clientFormFactor": "{}",
        "configInfo": {{
          "appInstallData": ""
        }},
        "acceptHeader": "*/*",
        "deviceExperimentId": ""
      }}
    }}{}
  }}
  "#,  lang, context.user_agent, context.client_name, context.client_version, context.os_name, context.os_version, context.platform, context.form_factor, map_to_json_part(&extra_options));
  let url = format!("{}{}{}/{}?key={}", WEBSITE_BASE_URL, INNERTUBE_API_URL, context.api_version, endpoint, context.api_key);
  let client = reqwest::Client::new();
  match client.post(url).body(String::from(request_string)).header("x-youtube-client-version", &context.client_version).send().await {
    Ok(response) => {
      match response.text().await {
        Ok(response_text) => {
          Ok(response_text)
        },
        Err(err) => Err(err)
      }
    },
    Err(err) => Err(err)
  }
}
