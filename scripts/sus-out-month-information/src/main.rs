
use shared::load_json;
use shared::DateInformation;
use yayti::parsers::web::Language;
use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};
use yayti::parsers::ClientContext;
use yayti::extractors::*;
use yayti::parsers::web::*;
use chrono::NaiveDateTime;
use std::{thread, time};
use regex::Regex;

#[tokio::main]
async fn main() {
  match load_json::<Vec::<(Language, Vec::<DateInformation>)>>("../../data/month-map.json") {
    Ok(date_information) => {
      let mut month_names = Vec::<Vec::<String>>::new();
      for i in 0..12 {
        month_names.push(Vec::<String>::new());
      }
      let mut language_month_map = serde_json::Map::new();
      let days = vec!("25", "27", "23", "23", "12", "16", "17", "14", "29", "17", "18", "15");
      for i in 0..date_information.len() {
        let mut month_names_for_language = Vec::<String>::new();
        let (language, dates) = &date_information[i];
        for k in 0..dates.len() {
          let date = dates[k].clone();
          let re = Regex::new(&format!(r"{}[^ ]*", days[k])).unwrap();
          let mut month_name = format!("{}", re.replace(&date.day_month, ""));
          if month_name == "" {
            month_name = date.day_month.replace(days[k], "")
          }
          month_names[k].push(String::from(&month_name));
          month_names_for_language.push(String::from(&month_name));
        }
        language_month_map.insert(String::from(&language.language_code), serde_json::json!(month_names_for_language));
      }
      match serde_json::to_string_pretty(&month_names) {
        Ok(output_json) => {
          let mut output = File::create("../../data/month-names.json").unwrap();
          write!(output, "{}", output_json);
        },
        Err(_) => {
          println!("Error: issue with formatting output")
        }
      };
      match serde_json::to_string_pretty(&language_month_map) {
        Ok(output_json) => {
          let mut output = File::create("../../data/language-month-map.json").unwrap();
          write!(output, "{}", output_json);
        },
        Err(_) => {
          println!("Error: issue with formatting output")
        }
      }
    },
    Err(error) => {
      println!("ERROR: {}", error);
    }
  }/*
  let seconds = 10;
  let milliseconds = time::Duration::from_millis(1000 * seconds);
  let languages = match load_json::<Vec::<Language>>("../../data/languages.json") {
    Ok(languages) => languages.into_iter().rev().collect::<Vec::<Language>>(),
    Err(_) => Vec::new()
  };
  let mut successful_codes = Vec::<String>::new();
  let mut error_codes = Vec::<String>::new();
  let videos_list = match load_json::<Vec::<String>>("../../data/month-videos.testing.json") {
    Ok(month_videos) => month_videos,
    Err(_) => Vec::new()
  };

  let published_timestamps = vec!(1578009600, 1488153600, 1678752000, 1681689600, 1620432000, 1592611200, 1658188800, 1659398400, 1283817600, 1634428800, 1290038400, 1671840000);
  for i in 0..languages.len() {
    let lang = &languages[i].language_code;
    for k in 0..videos_list.len() {
      let video = &videos_list[k];
      match innertube::fetch_next(video, &ClientContext::default_web(), Some(lang)).await {
        Ok(next_res) => {
          let json = serde_json::from_str::<serde_json::Value>(&next_res).unwrap();
          let published = video::get_published(&json, lang);
          match published {
            Some(published) => {
              let date = NaiveDateTime::from_timestamp_opt(published, 0).unwrap();
              println!("{}", published);
              println!("{}", date.format("%Y-%m-%d"));
              if published != published_timestamps[k] {
                println!("ERROR with lang: {} on video {}", lang, video);
                error_codes.push(String::from(lang));
              } else {
                successful_codes.push(String::from(lang));
              }
              println!("So far: {} correct - {} wrong", successful_codes.len(), error_codes.len());
            },
            None => {
              println!("SEVERE ERROR with lang: {} on video {}", lang, video);
            }
          }

        },
        Err(error) => {
          println!("Error fetching next: {}", error);
        }
      };
      thread::sleep(milliseconds);
    }
  }

  match serde_json::to_string_pretty(&successful_codes) {
    Ok(output_json) => {
      let mut output = File::create("../../data/success.json").unwrap();
      write!(output, "{}", output_json);
    },
    Err(_) => {
      println!("Error: issue with formatting output")
    }
  };

  match serde_json::to_string_pretty(&error_codes) {
    Ok(output_json) => {
      let mut output = File::create("../../data/error.json").unwrap();
      write!(output, "{}", output_json);
    },
    Err(_) => {
      println!("Error: issue with formatting output")
    }
  };*/
}
