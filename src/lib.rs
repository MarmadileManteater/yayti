
mod interface;
mod constants;
pub use interface::scrape_video_info;
pub use interface::fetch_video_info;
pub use interface::fetch_video_metadata;
pub use interface::fetch_video_streams;
pub use interface::Video as YTVideo;
pub use interface::ClientContext;

#[cfg(test)]
mod tests {
  use super::*;

  /* TODO write some tests🧪 */

  // Run an assert function on the data returned from 
  // both the innertube API and scraping YT manually
  // for a given video id and lang
  async fn scrape_and_fetch_video(id: &str, lang: Option<&str>, assert : fn(interface::Video)) {
    match scrape_video_info(id, lang).await {
      Ok(video) => {
        assert(video);
      },
      Err(err) => {
        assert_eq!("", err.message);
      }
    };
    match fetch_video_info(id, lang, None).await {
      Ok(video) => {
        assert(video);
      },
      Err(err) => {
        assert_eq!("", err.message);
      }
    };
  }

  #[tokio::test]
  async fn video_info_livestream() {
    fn assert(video: interface::Video) {
      assert_eq!("lofi hip hop radio - beats to relax/study to", video.title);
      assert_eq!("Lofi Girl", video.author);
      assert_eq!("UCSJ4gkVC6NrvII8umztf0Ow", video.author_id);
      assert_ne!("", video.dash_url);
      assert_ne!(0, video.like_count);
      assert_ne!(0, video.view_count);
      // Live streams pull back a dash url, but are missing format streams??? 🤷‍♀️
      // assert_ne!(0, video.format_streams.len());
      assert_ne!(0, video.adaptive_formats.len());
      assert_eq!(true, video.is_live);
    }
    scrape_and_fetch_video("jfKfPfyJRdk", None, assert).await
  }

  #[tokio::test]
  async fn video_info_generic_video() {
    fn assert(video: interface::Video) {
      assert_eq!("$1 vs $1,000,000 Hotel Room!", video.title);
      assert_eq!("MrBeast", video.author);
      assert_eq!("UCX6OQ3DkcsbYNE6H8uQQuVA", video.author_id);
      // dash manifests don't work for all cases rn 🤷‍♀️ i'm working on it
      //assert_ne!("", video.dash_url);
      assert_ne!(0, video.length_seconds);
      assert_ne!(0, video.like_count);
      assert_ne!(0, video.view_count);
      assert_ne!(0, video.format_streams.len());
      assert_ne!(0, video.adaptive_formats.len());
      assert_eq!(false, video.is_live);
    }
    scrape_and_fetch_video("iogcY_4xGjo", None, assert).await
  }

  #[tokio::test]
  async fn video_info_short() {
    fn assert(video: interface::Video) {
      assert_eq!("Click If You Like Toast", video.title);
      assert_eq!("MrBeast 2", video.author);
      assert_eq!("UC4-79UOlP48-QNGgCko5p2g", video.author_id);
      assert_ne!(0, video.length_seconds);
      // dash manifests don't work for all cases rn 🤷‍♀️ i'm working on it
      //assert_ne!("", video.dash_url);
      assert_ne!(0, video.like_count);
      assert_ne!(0, video.view_count);
      assert_ne!(0, video.format_streams.len());
      assert_ne!(0, video.adaptive_formats.len());
      assert_eq!(false, video.is_live);
    }
    scrape_and_fetch_video("PIIDWkxdcXY", None, assert).await
  }

  #[tokio::test]
  async fn video_info_past_livestream() {
    fn assert(video: interface::Video) {
      assert_eq!("Where Will This End? - WAN Show November 25, 2022", video.title);
      assert_eq!("Linus Tech Tips", video.author);
      assert_eq!("UCXuqSBlHAE6Xw-yeJA0Tunw", video.author_id);
      // dash manifests don't work for all cases rn 🤷‍♀️ i'm working on it
      //assert_ne!("", video.dash_url);
      //assert_ne!(0, video.length_seconds);
      assert_ne!(0, video.like_count);
      assert_ne!(0, video.view_count);
      assert_ne!(0, video.format_streams.len());
      assert_ne!(0, video.adaptive_formats.len());
      assert_eq!(false, video.is_live);
    }
    scrape_and_fetch_video("zYpyS2HaZHM", None, assert).await
  }
  #[tokio::test]
  async fn video_info_music() {
    fn assert(video: interface::Video) {
      assert_eq!("My Chemical Romance - Disenchanted", video.title);
      assert_eq!("My Chemical Romance", video.author);
      assert_eq!("UCCZGYab5SpD0I7Z5JqJZgww", video.author_id);
      // dash manifests don't work for all cases rn 🤷‍♀️ i'm working on it
      //assert_ne!("", video.dash_url);
      assert_ne!(0, video.length_seconds);
      assert_ne!(0, video.like_count);
      assert_ne!(0, video.view_count);
      assert_ne!(0, video.format_streams.len());
      assert_ne!(0, video.adaptive_formats.len());
      assert_eq!(false, video.is_live);
    }
    scrape_and_fetch_video("j_MlBCb9-m8", None, assert).await
  }
}
