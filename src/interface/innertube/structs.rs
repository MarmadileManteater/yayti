
use super::super::super::constants::{DEFAULT_WEB_API_KEY, DEFAULT_WEB_CVER};

pub struct ClientContext {
  pub api_key: String,
  pub client_version: String,
  pub user_agent: String,
  pub client_name: String,
  pub os_name: String,
  pub os_version: String,
  pub platform: String
}

impl ClientContext {
  pub fn default_web() -> ClientContext {
    ClientContext {
      api_key: String::from(DEFAULT_WEB_API_KEY),
      client_version: String::from(DEFAULT_WEB_CVER),
      user_agent: String::from("gzip(gfe)"),
      client_name: String::from("WEB"),
      os_name: String::from(""),
      os_version: String::from(""),
      platform: String::from("DESKTOP")
    }
  }
}
