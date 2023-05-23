
use std::string::FromUtf16Error;
use crate::constants::YT_THUMBNAIL_HOST_URL;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Thumbnail {
  pub quality: String,
  pub url: String,
  pub width: i32,
  pub height: i32
}

#[derive(Deserialize, Serialize)]
pub struct Size {
  pub name: String,
  pub label: String,
  pub width: i32,
  pub height: i32
}

pub fn generate_yt_video_thumbnail_url(id: &str, thumbnail_path: &str) -> String {
  return format!("{}/{}/{}.jpg", YT_THUMBNAIL_HOST_URL, id, thumbnail_path)
}

pub fn generate_yt_video_thumbnails(id: &str) -> Vec::<Thumbnail> {
  generate_yt_video_thumbnails_within_max_size(id, 1280)
}

pub fn generate_yt_video_thumbnails_within_max_size(id: &str, max_width: i32) -> Vec::<Thumbnail> {
  let known_thumbnail_sizes = [
    Size { name: String::from("maxres"), label: String::from("maxres"), width: 1280, height: 720 },
    Size { name: String::from("sd"), label: String::from("sd"), width: 640, height: 480 },
    Size { name: String::from("hq"), label: String::from("high"), width: 480, height: 360 },
    Size { name: String::from("mq"), label: String::from("medium"), width: 320, height: 180 },
    Size { name: String::from(""), label: String::from(""), width: 120, height: 90 }
  ];
  let mut thumbnails = Vec::with_capacity(10);
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
      let url = format!("{}/{}/{}.jpg", YT_THUMBNAIL_HOST_URL, id, format!("{}{}", size.name, thumbnail_type));
      if size.width < max_width {
        thumbnails.push(Thumbnail {
          quality: format_name,
          url: url,
          width: size.width,
          height: size.height
        });
      }
    }
  };
  thumbnails
}

pub struct SubVectorError {
  pub message: String
}
impl SubVectorError {
  fn new(message: &str) -> SubVectorError {
    SubVectorError {
      message: String::from(message)
    }
  }
}

// is the equivalent of .substring(), but for Vec::<T>.
trait SubVector<T: Copy> {
  fn subvector(&self, start: usize, end: usize) -> Result<Vec::<T>,  SubVectorError>;
}
impl<T: Copy> SubVector<T> for Vec::<T> {
  fn subvector(&self, start: usize, end: usize) -> Result<Vec::<T>, SubVectorError> {
    let mut new_vector = Vec::<T>::new();
    let length = self.len();
    if length < start {
      Err(SubVectorError::new("Invalid start index"))
    } else if length < end {
      Err(SubVectorError::new("Invalid end index"))
    } else if end < start {
      Err(SubVectorError::new("Start must come before end"))
    } else {
      for i in start..end {
        new_vector.push(self[i]);
      }
      Ok(new_vector)
    }
  }
}

pub struct UTF16SubStringError {
  pub message: String,
  pub inner_sub_string_error: Option<SubVectorError>,
  pub inner_utf_16_error: Option<FromUtf16Error>
}
impl UTF16SubStringError {
  fn new_sub_string_error(message: &str, error: SubVectorError) -> UTF16SubStringError {
    UTF16SubStringError {
      message: String::from(message),
      inner_sub_string_error: Some(error),
      inner_utf_16_error: None
    }
  }
  fn new_utf_16_error(message: &str, error: FromUtf16Error) -> UTF16SubStringError {
    UTF16SubStringError {
      message: String::from(message),
      inner_sub_string_error: None,
      inner_utf_16_error: Some(error)
    }
  }
}

// helper trait for taking the substring of the utf-16 encoded attributed description
pub trait UTF16Substring{
  fn subvector_as_str(&self, start: usize, end: usize) -> Result<String, UTF16SubStringError>;
}
impl UTF16Substring for Vec::<u16> {
  fn subvector_as_str(&self, start: usize, end: usize) -> Result<String, UTF16SubStringError> {
    match self.subvector(start as usize, end as usize) {
      Ok(contents) => {
        match String::from_utf16(&contents) {
          Ok(string_contents) => Ok(string_contents),
          Err(err) => {
            Err(UTF16SubStringError::new_utf_16_error("Error converting utf-16 vector to string", err))
          }
        }
      },
      Err(err) => Err(UTF16SubStringError::new_sub_string_error("Error taking subvector of utf-16 vector", err))
    }
  }
}
