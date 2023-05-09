use yayti::extractors::innertube::fetch_account_menu;
use yayti::parsers::{ClientContext,web::get_languages};
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() {
  match fetch_account_menu(&ClientContext::default_web(), Some("en")).await {
    Ok(account_menu_res) => {
      let json = serde_json::from_str::<serde_json::Value>(&account_menu_res);
      match json {
        Ok(json) => {
          match get_languages(&json) {
            Some(languages) => {
              match serde_json::to_string_pretty(&languages) {
                Ok(languages_json) => {
                  let mut output = File::create("../../data/languages.json").unwrap();
                  write!(output, "{}", languages_json);
                },
                Err(_) => {
                  println!("Error: issue with formatting output")
                }
              }
            },
            None => {
              println!("Error: get languages returned None!")
            }
          }
        },
        Err(error) => {
          println!("{}", error);
        }
      }
    },
    Err(err) => {
      println!("Error: {}", err.source().unwrap().to_string());
    }
  };
}
