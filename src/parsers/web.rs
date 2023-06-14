
use serde_json::Value;
use serde::{Serialize, Deserialize};

pub mod video;
pub mod playlist;

#[derive(Deserialize, Serialize)]
pub struct Language {
  pub language_name: String,
  pub language_code: String
}

pub fn get_languages(json: &Value) -> Option<Vec<Language>> {
  match json["actions"][0]["openPopupAction"]["popup"]["multiPageMenuRenderer"]["sections"][1]["multiPageMenuSectionRenderer"]["items"][1]["compactLinkRenderer"]["serviceEndpoint"]["signalServiceEndpoint"]["actions"][0]["getMultiPageMenuAction"]["menu"]["multiPageMenuRenderer"]["sections"][0]["multiPageMenuSectionRenderer"]["items"].as_array() {
    Some(language_items) => {
      Some(language_items.into_iter().filter_map(|language_item| {
        match language_item["compactLinkRenderer"]["title"]["simpleText"].as_str() {
          Some(language_name) => {
            match language_item["compactLinkRenderer"]["serviceEndpoint"]["signalServiceEndpoint"]["actions"][0]["selectLanguageCommand"]["hl"].as_str() {
              Some(language_code) => {
                Some(Language {
                  language_name: String::from(language_name),
                  language_code: String::from(language_code)
                })
              },
              None => {
                None
              }
            }
          },
          None => {
            None
          }
        }
      }).collect::<Vec<Language>>())
    },
    None => None
  }
}
