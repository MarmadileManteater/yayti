
use std::fs::File;
use std::io::Read;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct DateInformation {
  pub day_month: String,
  pub year: String,
  pub date_text: String
}

pub fn load_json<T : DeserializeOwned>(path: &str) -> Result<T, String> {
  match File::open(path) {
    Ok(mut file) => {
      let mut contents = String::new();
      match file.read_to_string(&mut contents) {
        Ok(_) => {
          let str_contents = &contents;
          match serde_json::from_str::<T>(&String::from(str_contents)) {
            Ok(result) => Ok(result),
            Err(_) => Err(String::from("problem deserializing string"))
          }
        },
        Err(_) => Err(String::from("problem reading file"))
      }
    },
    Err(_) => Err(String::from("problem opening file"))
  }
}
