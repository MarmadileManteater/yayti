
# yet another yt interface

This library provides an interface for YT which attempts to return data in a similar format to the Invidious API while offering the ability to either scrape YT or use the innertube API.

## 🛠 basic usage
```rust
use yayti::{scrape_video_info, fetch_video_info};

async fn some_function_name(id: &str) {
  // Uses the innertube API to retrieve the YT video data
  // Optionally, the default API key and cver can be overridden
  match fetch_video_info(id, Some("en"), None, None).await {
    Ok(video) => {
      println!("Video Title: {}", video.title);
      println!("Author Name: {}", video.author);
    },
    Err(err) => {
      println!("{}", err.message);
    }
  };
  // Scrapes the YT video data off of the video page
  match scrape_video_info(id, Some("en")).await {
    Ok(video) => {
      println!("Video Title: {}", video.title);
      println!("Author Name: {}", video.author);
    },
    Err(err) => {
      println!("{}", err.message);
    }
  };
}
```
