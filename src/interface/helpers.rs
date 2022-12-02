
use super::structs::{Thumbnail, Size};

pub fn generate_yt_video_thumbnails(id: &str) -> Vec::<Thumbnail> {
  let known_thumbnail_sizes = [
    Size { name: String::from("maxres"), label: String::from("maxres"), width: 1280, height: 720 },
    Size { name: String::from("sd"), label: String::from("sd"), width: 640, height: 480 },
    Size { name: String::from("hq"), label: String::from("high"), width: 480, height: 360 },
    Size { name: String::from("mq"), label: String::from("medium"), width: 320, height: 180 },
    Size { name: String::from(""), label: String::from(""), width: 120, height: 90 }
  ];
  let mut thumbnails = Vec::with_capacity(10);
  thumbnails.push(Thumbnail {
    quality: String::from("maxres"),
    url: format!("https://i.ytimg.com/vi/{}/maxresdefault.jpg", id),
    width: 1280,
    height: 720
  });
  let known_thumbnail_types = ["default", "1", "2", "3"];
  for size in known_thumbnail_sizes {
    for thumbnail_type in known_thumbnail_types {
      let format_name = match thumbnail_type {
        "1" => {
          format!("{}start", size.name)
        },
        "2" => {
          format!("{}middle", size.name)
        },
        "3" => {
          format!("{}end", size.name)
        },
        "default" => {
          match &size.name as &str {
            "hq" => {
              String::from("high")
            },
            "mq" => {
              String::from("medium")
            },
            &_ => {
              format!("{}{}", size.name, thumbnail_type)
            }
          }
        }
        &_ => {
          format!("{}{}", size.name, thumbnail_type)
        }
      };
      let url = format!("https://i.ytimg.com/vi/{}/{}.jpg", id, format!("{}{}", size.name, thumbnail_type));
      thumbnails.push(Thumbnail {
        quality: format_name,
        url: url,
        width: size.width,
        height: size.height
      });
    }
  };
  thumbnails
}
