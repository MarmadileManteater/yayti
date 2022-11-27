
mod video;
use serde_json::Value;

pub use video::ScrapeVideoError;

pub async fn get_video_info(id: &str, lang: Option<&str>) -> Result<Value, ScrapeVideoError> {
  video::get_info(id, lang).await
}
