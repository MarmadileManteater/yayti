
use super::super::super::constants::{DEFAULT_WEB_API_KEY, DEFAULT_WEB_CVER};
use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Debug, Deserialize, Serialize)]
pub struct ClientContext {
  pub api_key: String,
  pub api_version: String,
  pub client_version: String,
  pub user_agent: String,
  pub client_name: String,
  pub os_name: String,
  pub os_version: String,
  pub platform: String,
}

impl ClientContext {
  pub fn default_web() -> ClientContext {
    ClientContext {
      api_key: String::from(DEFAULT_WEB_API_KEY),
      api_version: String::from("v1"),
      client_version: String::from(DEFAULT_WEB_CVER),
      user_agent: String::from("gzip(gfe)"),
      client_name: String::from("WEB"),
      os_name: String::from(""),
      os_version: String::from(""),
      platform: String::from("DESKTOP")
    }
  }
  pub fn from_json(j_object: &Value) -> ClientContext {
    let defaults = ClientContext::default_web();
    let innertube_api_key = match j_object["INNERTUBE_API_KEY"].as_str() {
      Some(innertube_api_key) => String::from(innertube_api_key),
      None => defaults.api_key
    };
    let innertube_api_version = match j_object["INNERTUBE_API_VERSION"].as_str() {
      Some(innertube_api_version) => String::from(innertube_api_version),
      None => defaults.api_version
    };
    let innertube_client_version = match j_object["INNERTUBE_CLIENT_VERSION"].as_str() {
      Some(innertube_client_version) => String::from(innertube_client_version),
      None => defaults.client_version
    };
    let user_agent = match j_object["INNERTUBE_CONTEXT"]["client"]["userAgent"].as_str() {
      Some(user_agent) => String::from(user_agent),
      None => defaults.user_agent
    };
    let client_name = match j_object["INNERTUBE_CLIENT_NAME"].as_str() {
      Some(client_name) => String::from(client_name),
      None => defaults.client_name
    };
    let os_name = match j_object["INNERTUBE_CONTEXT"]["client"]["osName"].as_str() {
      Some(os_name) => String::from(os_name),
      None => defaults.os_name
    };
    let os_version = match j_object["INNERTUBE_CONTEXT"]["client"]["osVersion"].as_str() {
      Some(os_version) => String::from(os_version),
      None => defaults.os_version
    };
    let platform = match j_object["INNERTUBE_CONTEXT"]["client"]["platform"].as_str() {
      Some(platform) => String::from(platform),
      None => defaults.platform
    };

    ClientContext {
      api_key: innertube_api_key,
      api_version: innertube_api_version,
      client_version: innertube_client_version,
      user_agent: user_agent,
      client_name: client_name,
      os_name: os_name,
      os_version: os_version,
      platform: platform
    }
  }
}
