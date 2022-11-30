

mod scrape;
mod structs;
mod innertube;
use log::warn;

pub use structs::Video;
pub use innertube::ClientContext;

// ✂ Scrape the video data from YT
pub async fn scrape_video_info(id: &str, lang: Option<&str>) -> Result<structs::Video, scrape::ScrapeVideoError> {
  match scrape::get_video_info(id, lang).await {
    Ok(video_info_json) => {
      // parse out the API context from this JSON
      Ok(structs::Video::new(video_info_json))
    },
    Err(err) => Err(err)
  }
}

// 📡 Retrieve the video data from each of the specified endpoints of the innertube API
async fn fetch_video_endpoints(id: &str, lang: Option<&str>, client_context : Option<innertube::ClientContext>, include_next : bool, include_player: bool) -> Result<structs::Video, innertube::InnertubeVideoError> {
  let client_context = match client_context {
    Some(client_context) => client_context,
    None => ClientContext::default_web()// default to the web context
  };
  let next_body = if include_next {
    match innertube::get_video_metadata(&client_context, id, lang).await {
      Ok(body) => body,
      Err(err) => {
        warn!("{}", err.message);
        String::from("{}")
      }
    }
  } else {
    String::from("{}")
  };
  let player_body = if include_player {
    match innertube::get_video_streams(&client_context, id, lang).await {
      Ok(body) => body,
      Err(err) => {
        warn!("{}", err.message);
        String::from("{}")
      }
    }
  } else {
    String::from("{}")
  };
  match serde_json::from_str::<serde_json::Value>(&format!("{{ \"next\": {}, \"player\": {} }}", next_body, player_body)) {
    Ok(json) => {
      Ok(structs::Video::new(json))
    },
    Err(error) => {
      Err(innertube::InnertubeVideoError { message: String::from("Failed"), inner_reqwest_error: None, inner_serde_error: Some(error) })
    }
  }
}

// 📡 Retrieve the video data from the innertube API
pub async fn fetch_video_info(id: &str, lang: Option<&str>, client_context: Option<innertube::ClientContext>) -> Result<structs::Video, innertube::InnertubeVideoError> {
  // Fetch all of the available endpoints from the innertube API
  fetch_video_endpoints(id, lang, client_context, true, true).await
}

// 📡 Retrieve the video metadata from the innertube API
pub async fn fetch_video_metadata(id: &str, lang: Option<&str>, client_context: Option<innertube::ClientContext>) -> Result<structs::Video, innertube::InnertubeVideoError> {
  // Fetch the next endpoint from the innertube API
  fetch_video_endpoints(id, lang, client_context, true, false).await
}

// 📡 Retrieve the video streams from the innertube API
pub async fn fetch_video_streams(id: &str, lang: Option<&str>, client_context: Option<innertube::ClientContext>) -> Result<structs::Video, innertube::InnertubeVideoError> {
  // Fetch the player endpoint from the innertube API
  fetch_video_endpoints(id, lang, client_context, false, true).await
}
