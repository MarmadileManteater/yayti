
use yayti::parsers::web::Language;
use yayti::parsers::ClientContext;
use yayti::extractors::innertube::fetch_next;
use yayti::parsers::web::*;
use std::fs::File;
use std::io::Read;
use std::{thread, time};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io::Write;
use chrono::Utc;
use chrono::Duration;
use shared::load_json;
use shared::DateInformation;
use regex::Regex;

#[tokio::main]
async fn main() {
  match load_json::<Vec::<Language>>("../../data/languages.json") {
    Ok(languages) => {
      match load_json::<Vec::<String>>("../../data/month-videos.json") {
        Ok(month_videos) => {
          let dt = Utc::now();
          println!("starting at: {}", dt);
          let seconds = 4;
          let milliseconds = time::Duration::from_millis(1000 * seconds);
          let mut results = Vec::<(&Language, Vec::<DateInformation>)>::new();
          for i in 0..languages.len() {
            let language = &languages[i];
            results.push((language, Vec::<DateInformation>::new()));
            println!("{}: {}", language.language_name, language.language_code);
            for k in 0..month_videos.len() {
              println!("fetching: {}", &month_videos[k]);
              match fetch_next(&month_videos[k], &ClientContext::default_web(), Some(&language.language_code)).await {
                Ok(next_res) => {
                  match serde_json::from_str::<serde_json::Value>(&next_res) {
                    Ok(json) => {
                      match video::get_title(&json) {
                        Some(title) => println!("Title: {}", title),
                        None => {}
                      };
                      match video::get_date_parts(&json) {
                        Some((label, value)) => {
                          let date_text = match video::get_date_text(&json) {
                            Some(date_text) => date_text,
                            None => {
                              println!("⚠ WARN: No date-text found!");
                              String::from("")
                            }
                          };
                          // let's just say we can assume that this code will not be functional in 100 years anyway
                          let year = match Regex::new(r"20[0-9][0-9]").unwrap().captures(&label) {
                            Some(_) => String::from(&label),
                            None => String::from(&value)
                          };
                          let day_month = if year == label {
                            String::from(&value)
                          } else {
                            String::from(&label)
                          };
                          results[i].1.push(DateInformation {
                            day_month,
                            year,
                            date_text
                          });
                        },
                        None => {
                          println!("No date-parts found!");
                        }
                      }
                    },
                    Err(_) => {
                      println!("Error serializing next response");
                    }
                  }
                },
                Err(error) => {
                  println!("Error: {}`", error);
                }
              }
              let mut output = File::create("../../data/month-map.json").unwrap();
              write!(output, "{}", serde_json::to_string_pretty(&results).unwrap());
              thread::sleep(milliseconds);
            }
            thread::sleep(milliseconds);
          }
          // This is going to be a tonne of requests, so this should be staggered to avoid ratelimiting
          // Like 12 x languages.len() requests
          // 
        },
        Err(error) => {
          println!("Error: {}`", error);
        }
      }
    },
    Err(error) => {
      println!("Error: {}`", error);
    }
  }
}
