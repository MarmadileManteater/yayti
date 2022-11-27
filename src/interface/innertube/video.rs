
#[derive(Debug)]
pub struct InnertubeVideoError {
  pub message: String,
  pub inner_reqwest_error: Option<reqwest::Error>,
  pub inner_serde_error: Option<serde_json::Error>
}

// 📡Reaches out to a given video endpoint for the 📺innertube api
async fn get_video_endpoint(endpoint: &str, client_version: &str, api_key: &str, id : &str, lang : Option<&str>) -> Result<String, InnertubeVideoError> {
  let lang = match lang {
    Some(lang) => lang,
    None => "en"
  };
  let meta_data_request_string = format!(r#"
  {{
    "context": {{
      "client": {{
        "hl": "{}",
        "gl": "US",
        "remoteHost": "",
        "deviceMake": "",
        "deviceModel": "",
        "visitorData": "",
        "userAgent": "gzip(gfe)",
        "clientName": "WEB",
        "clientVersion": "{}",
        "osName": "",
        "osVersion": "",
        "originalUrl": "",
        "platform": "DESKTOP",
        "clientFormFactor": "UNKNOWN_FORM_FACTOR",
        "configInfo": {{
          "appInstallData": ""
        }},
        "acceptHeader": "*/*",
        "deviceExperimentId": ""
      }},
      "user": {{
        "lockedSafetyMode": false
      }},
      "request": {{
        "useSsl": true
      }},
      "clickTracking": {{
        "clickTrackingParams": ""
      }}
    }},
    "videoId": "{}",
    "racyCheckOk": false,
    "contentCheckOk": false,
    "autonavState": "STATE_NONE",
    "playbackContext": {{
      "vis": 0,
      "lactMilliseconds": "-1"
    }}
  }}
  "#, lang, client_version, id);
  let client = reqwest::Client::new();
  match client.post(format!("https://www.youtube.com/youtubei/v1/{}?key={}", endpoint, api_key)).body(String::from(meta_data_request_string)).send().await {
    Ok(next_response) => {
      match next_response.text().await {
        Ok(next_response_text) => Ok(next_response_text),
        Err(err) => {
          Err(InnertubeVideoError { message: String::from("Next response not found"), inner_reqwest_error: Some(err), inner_serde_error: None })
        }
      }
    },
    Err(err) => {
      Err(InnertubeVideoError { message: String::from("Next response not found"), inner_reqwest_error: Some(err), inner_serde_error: None })
    }
  }
}

// 📡Retrieves the video metadata from 📺innertube api
pub async fn get_video_metadata(client_version: &str, api_key: &str, id : &str, lang : Option<&str>) -> Result<String, InnertubeVideoError> {
  get_video_endpoint("next", client_version, api_key, id, lang).await
}

// 📡Retrieves the video streams from 📺innertube api
pub async fn get_video_streams(client_version: &str, api_key: &str, id : &str, lang : Option<&str>) -> Result<String, InnertubeVideoError> {
  get_video_endpoint("player", client_version, api_key, id, lang).await
}
