
# yet another yt interface

This library provides an interface for YT which attempts to return data in a similar format to the [Invidious](https://github.com/iv-org/invidious) API while offering the ability to either scrape YT or use the innertube API.

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

# 👩‍🏭 progress

[-] 🎥 videos
  - At this time, `yayti::{scrape_video_info, fetch_video_info}` exposes most of the video information except for the related videos and the storyboards. It isn't that the data isn't there, but I just haven't mapped the data to the output yet.
[ ] 🧑 channels
[ ] 📼 playlists
[ ] 🗨 comments

# purpose

I created this to 🧠learn more about the way extracting data from YT currently works in some existing javascript libraries: [Youtube.js](https://github.com/LuanRT/YouTube.js) and [node-ytdl-core](https://github.com/fent/node-ytdl-core). I also did a little bit of my own 🔍research by monitoring what requests were made on YT page transitions. I would say that most of the implementation I've written so far is fairly naive, but hey, it kind of works, and that's 😎cool.
