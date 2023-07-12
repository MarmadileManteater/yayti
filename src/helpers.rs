
use std::string::{FromUtf16Error, FromUtf8Error};
use base64::engine::general_purpose;
use regex::Regex;
use shared::MonthInformation;
use std::str::FromStr;
use rand::seq::SliceRandom;
use chrono::{NaiveDate, NaiveTime, ParseError, Utc};
use log::{warn};
use crate::{constants::YT_THUMBNAIL_HOST_URL, proto::{PageCountData, InnerPlaylistContinuationData, InnerPlaylistContinuation, PlaylistContinuation, VisitorData, PlayerParams}};
use serde_json::{from_str,Value, to_string};
use serde::{Deserialize, Serialize};
use prost::{Message, EncodeError};
use base64::Engine;

#[derive(Deserialize, Serialize, Clone)]
pub struct Thumbnail {
  pub quality: String,
  pub url: String,
  pub width: i32,
  pub height: i32
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AuthorThumbnail {
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
      if size.width <= max_width {
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

pub enum ParseDateError {
  NoYearFoundInDate,
  ErrorDeserializingMapBytes(serde_json::Error),
  LanguageNotFound,
  ErrorParsingFinalDateString(ParseError),
  ErrorConvertingYearToNum
}

pub enum ParseDateOption {
  ParseDayMonth(String, String),
  ParseFullDate(String)
}

// Attempts to sus out a date from a lang string given the lang
#[cfg(feature = "parse_languages_to_published")]
pub fn parse_date_to_published(lang: &str, option: &ParseDateOption) -> Result<i64, ParseDateError> {
  use substring::Substring;
  let some_number = Regex::new(r"[0-9]+").unwrap();
  // let's just say we can assume that this code will not be functional in 100 years anyway
  let match_years = Regex::new(r"20[0-9][0-9]").unwrap();
  let (year, day_month) = match option {
    ParseDateOption::ParseDayMonth(year, day_month) => {
      (String::from(year), String::from(day_month))
    },
    ParseDateOption::ParseFullDate(date) => {
      let year = match match_years.captures(date) {
        Some(captures) => {
          captures.get(0).unwrap().as_str()
        },
        None => {
          return Err(ParseDateError::NoYearFoundInDate)
        }
      };
      // Remove year from date string to acquire the day-month
      let day_month = date.replace(&format!("{}", year), "");
      (String::from(year), String::from(&day_month))
    }
  };
  let month_map = match from_str::<Value>(include_str!("../data/language-month-map.json")) {
    Ok(map) => {
      match map[lang].as_array() {
        Some(months) => {
          months.clone()
        },
        None => {
          warn!("language pref not found; defaulting to english");
          if lang != "en" {
            match map["en"].as_array() {
              Some(months) => months.clone(),
              None => return Err(ParseDateError::LanguageNotFound)
            }
          } else {
            return Err(ParseDateError::LanguageNotFound)
          }
        }
      }
    },
    Err(error) => {
      return Err(ParseDateError::ErrorDeserializingMapBytes(error))
    }
  };
  let months = &month_map.into_iter().map(|value| from_str::<MonthInformation
    >(&to_string(&value).unwrap()).unwrap()).rev().collect::<Vec::<MonthInformation>>();
  let mut month = 12;
  let mut day = 1;
  for i in 0..months.len() {
    let month_name = match option {
      ParseDateOption::ParseDayMonth(_,_) => {
        &months[i].day_month_string
      },
      ParseDateOption::ParseFullDate(_) => {
        &months[i].full_date_string
      }
    };
    if day_month.contains(month_name) || day_month.contains(month_name.substring(0, month_name.len() - 1)) {
      month = months.len() - i;// add one because months are 1 indexed in dates
      let date_without_month = day_month.replace(month_name, "");
      match some_number.captures(&date_without_month) {
        Some(some_number) => {
          match i32::from_str(&some_number[0]) {
            Ok(number) => {
              day = number
            },
            Err(_) => {}
          };
          break;
        },
        None => {}
      };
    }
  }
  let year_num = match match_years.captures(&year) {
    Some(year_num_string) => {
      match i32::from_str(&year_num_string[0]) {
        Ok(year_num) => Some(year_num),
        Err(_) => None
      }
    },
    None => None
  };
  match year_num {
    Some(year) => {
      let date_string = format!("{}-{}-{}", year, month, day);
      match NaiveDate::parse_from_str(&date_string, "%Y-%m-%d") {
        Ok(date_time) => {
          // idk what is the best ⌚time value to get dates that are the same that come out of other yt extractors
          Ok(date_time.and_time(NaiveTime::from_hms_micro_opt(23, 59, 59, 0).unwrap()).timestamp())
        },
        Err(error) => Err(ParseDateError::ErrorParsingFinalDateString(error))
      }
    },
    None => Err(ParseDateError::ErrorConvertingYearToNum)
  }

}

pub fn decode_buffer<T:Message+Default>(proto_buf: Vec::<u8>) -> impl Message {
  T::decode(&proto_buf[..]).unwrap()
}

pub fn encode_buffer(continuation: impl Message) -> Result<Vec::<u8>, EncodeError> {
  let mut buf = [0u8; 200].to_vec();
  let mut buf_mut = buf[..].as_mut();
  match continuation.encode(&mut buf_mut) {
    Ok(_) => {},
    Err(error) => return Err(error)
  };
  Ok(buf.into_iter().filter_map(|data| if data != 0 { Some(data) } else { None }).collect::<Vec::<u8>>())
}

pub enum PlaylistContinuationError {
  EncodeError(EncodeError)
}

pub fn generate_playlist_continuation(playlist_id: &str, page_num: i32 /* 1 indexed */) -> Result<String, PlaylistContinuationError> {
  let inner_encoded = match encode_buffer(PageCountData {
    max: 100 * (page_num)
  }) {
    Ok(result) => result,
    Err(error) => return Err(PlaylistContinuationError::EncodeError(error))
  };
  let inner_encoded = match encode_buffer(InnerPlaylistContinuationData {
    some_random_junk_idk: 0,
    inner_page_count: format!("PT:{}", general_purpose::STANDARD.encode(&inner_encoded[..]))
  }) {
    Ok(result) => result,
    Err(error) => return Err(PlaylistContinuationError::EncodeError(error))
  };
  let encoded = match encode_buffer(PlaylistContinuation {
    inner: Some(InnerPlaylistContinuation  {
        id_full: format!("VL{}", playlist_id),
        inner_data: urlencoding::encode(&general_purpose::STANDARD.encode(&inner_encoded[..])).to_string(),
        id: String::from(playlist_id)
      })
  }) {
    Ok(result) => result,
    Err(error) => return Err(PlaylistContinuationError::EncodeError(error))
  };
  let base64_encoded = general_purpose::STANDARD.encode(&encoded[..]);
  let continuation = urlencoding::encode(&base64_encoded);
  Ok(format!("{}", continuation))
}

pub enum ProtobufError {
  EncodeError(EncodeError)
}

pub fn generate_visitor_data(visitor_id: &str) -> Result<String, ProtobufError> {
  let data = VisitorData {
    visitor_id: String::from(visitor_id),
    timestamp: Utc::now().timestamp()
  };
  match encode_buffer(data) {
    Ok(buffer) => {
      let base64_encoded = general_purpose::STANDARD.encode(&buffer[..]);
      let url_encoded = urlencoding::encode(&base64_encoded);
      Ok(format!("{}", url_encoded))
    },
    Err(e) => Err(ProtobufError::EncodeError(e))
  }
}

pub fn generate_random_noise(len: usize) -> String {
  let possible_characters = "0123456789qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM".split("").collect::<Vec::<&str>>();
  let mut output = Vec::with_capacity(len);
  for i in 0..len {
    output.push(possible_characters.choose(&mut rand::thread_rng()));
  }
  output.into_iter().map(|s| String::from(*s.unwrap_or(&""))).collect::<String>()
}

pub fn generate_player_params(num: i32) -> Result<String, ProtobufError> {
  match encode_buffer(PlayerParams {
    param: num
  }) {
    Ok(buffer) => {
      let base64_encoded = general_purpose::STANDARD.encode(&buffer[..]);
      let url_encoded = urlencoding::encode(&base64_encoded);
      Ok(format!("{}", url_encoded))
    },
    Err(e) => Err(ProtobufError::EncodeError(e))
  }
}
