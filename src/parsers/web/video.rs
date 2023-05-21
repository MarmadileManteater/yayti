
use serde_json::Value;
use urlencoding::{decode,encode};
use crate::constants::{SHORT_WEBSITE_BASE_URL, WEBSITE_BASE_URL};
use crate::helpers::{UTF16Substring};
use chrono::{ NaiveDate, NaiveTime};
use regex::Regex;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use serde_json::json;
#[cfg(feature = "decipher_streams")]
use super::super::ciphers::{create_formatable_decipher_js_code,format_decipher_code_into_executable, run_js_in_boa};

fn add_host_param_to_url(url: &str) -> String {
  let host_re = Regex::new(r"(.*?\.googlevideo\.com)").unwrap();
  match host_re.captures(url) {
    Some (host_captures) => {
      let host = host_captures.get(1).unwrap().as_str();
      format!("{}&host={}", url, encode(host))
    },
    None => {
      format!("{}", url)
    }
  }
}

#[cfg(feature = "parse_languages_to_published")]
pub fn fmt_inv(json: &Value, lang: &str) -> serde_json::Map<String, serde_json::Value> {
  let output = serde_json::Map::new();
  fmt_inv_with_existing_map(json, lang, output)
}

#[cfg(feature = "parse_languages_to_published")]
pub fn fmt_inv_with_existing_map(json: &Value, lang: &str, mut existing_map: serde_json::Map<String, serde_json::Value>) -> serde_json::Map<String, serde_json::Value> {
  if !existing_map.contains_key("title") {
    match get_title(json) {
      Some(title) => {
        existing_map.insert(String::from("title"), json!(title));
      },
      None => {}
    };
  }
  if !existing_map.contains_key("videoId") {
    match get_video_id(json) {
      Some(video_id) => {
        existing_map.insert(String::from("videoId"), json!(video_id));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("description") {
    match get_description(json) {
      Some(description) => {
        existing_map.insert(String::from("description"), json!(description));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("descriptionHtml") {
    match get_description_html(json) {
      Some(description_html) => {
        existing_map.insert(String::from("descriptionHtml"), json!(description_html));
      },
      None => {}
    };
  }
  if !existing_map.contains_key("published") {
    match get_published(json, lang) {
      Some(published) => {
        existing_map.insert(String::from("published"), json!(published));
      },
      None => match get_publish_date(json) {
        Some(published) => {
          existing_map.insert(String::from("published"), json!(published));
        },
        None => {}
      }
    };
  }
  if !existing_map.contains_key("publishedText") {
    match get_published_text(json) {
      Some(published_text) => {
        existing_map.insert(String::from("publishedText"), json!(published_text));
      },
      None => {}
    };
  }
  if !existing_map.contains_key("keywords") {
    match get_keywords(json) {
      Some(keywords) => {
        existing_map.insert(String::from("keywords"), json!(keywords));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("viewCount") {
    match get_view_count(json) {
      Some(view_count) => {
        existing_map.insert(String::from("viewCount"), json!(view_count));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("likeCount") {
    match get_like_count(json) {
      Some(like_count) => {
        existing_map.insert(String::from("likeCount"), json!(like_count));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("isFamilyFriendly") {
    match is_family_friendly(json) {
      Some(family_friendly) => {
        existing_map.insert(String::from("isFamilyFriendly"), json!(family_friendly));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("allowedRegions") {
    match get_available_countries(json) {
      Some(available_countries) => {
        existing_map.insert(String::from("allowedRegions"), json!(available_countries));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("genre") {
    match get_category(json) {
      Some(category) => {
        existing_map.insert(String::from("genre"), json!(category));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("author") {
    match get_author(json) {
      Some(author) => {
        existing_map.insert(String::from("author"), json!(author));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("authorId") {
    match get_author_id(json) {
      Some(author_id) => {
        existing_map.insert(String::from("authorId"), json!(author_id));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("authorUrl") {
    match get_author_id(json) {
      Some(author_id) => {
        existing_map.insert(String::from("authorUrl"), json!(format!("/channel/{}", author_id)));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("authorThumbnails") {
    match get_author_thumbnails(json) {
      Some(author_thumbnails) => {
        existing_map.insert(String::from("authorThumbnails"), json!(author_thumbnails));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("subCountText") {
    match get_sub_count_text(json) {
      Some(sub_count_text) => {
        existing_map.insert(String::from("subCountText"), json!(sub_count_text));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("lengthSeconds") {
    match get_length_seconds(json) {
      Some(length_seconds) => {
        existing_map.insert(String::from("lengthSeconds"), json!(length_seconds));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("allowRatings") {
    match get_allow_ratings(json) {
      Some(allow_ratings) => {
        existing_map.insert(String::from("allowRatings"), json!(allow_ratings));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("isListed") {
    match is_listed(json) {
      Some(is_listed) => {
        existing_map.insert(String::from("isListed"), json!(is_listed));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("liveNow") {
    match is_live_now(json) {
      Some(live_now) => {
        existing_map.insert(String::from("liveNow"), json!(live_now));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("hlsUrl") {
    match get_hls_url(json) {
      Some(hls_url) => {
        existing_map.insert(String::from("hlsUrl"), json!(hls_url));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("dashUrl") {
    match get_dash_url(json) {
      Some(dash_url) => {
        existing_map.insert(String::from("dashUrl"), json!(dash_url));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("formatStreams") {
    match get_legacy_formats(json) {
      Some(legacy_formats) => {
        existing_map.insert(String::from("formatStreams"), legacy_formats.into_iter().map(|format| format.into_inv()).collect::<Value>());
      },
      None => {}
    };
  }
  if !existing_map.contains_key("adaptiveFormats") {
    match get_adaptive_formats(json) {
      Some(formats) => {
        existing_map.insert(String::from("adaptiveFormats"), formats.into_iter().map(|format| format.into_inv()).collect::<Value>());
      },
      None => {}
    }
  }
  if !existing_map.contains_key("captions") {
    match get_captions(json) {
      Some(captions) => {
        existing_map.insert(String::from("captions"), json!(captions));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("recommendedVideos") {
    match get_recommended(json) {
      Some(videos) => {
        existing_map.insert(String::from("recommendedVideos"), videos.into_iter().filter_map(|format| format.into_inv()).collect::<Value>());
      },
      None => {}
    }
  }
  if !existing_map.contains_key("musicTracks") {
    match get_music_tracks(json) {
      Some(tracks) => {
        existing_map.insert(String::from("musicTracks"), tracks.into_iter().map(|track| track.into_inv()).collect::<Value>());
      },
      None => {}
    }
  }
  existing_map
}

#[cfg(feature = "parse_languages_to_published")]
#[cfg(feature = "decipher_streams")]
pub fn fmt_inv_and_decipher(json: &Value, lang: &str, player_res: &str) -> serde_json::Map<String, serde_json::Value> {
  let output = serde_json::Map::new();
  fmt_inv_with_existing_map_and_decipher(json, lang, player_res, output)
}

#[cfg(feature = "parse_languages_to_published")]
#[cfg(feature = "decipher_streams")]
pub fn fmt_inv_with_existing_map_and_decipher(json: &Value, lang: &str, player_res: &str, mut existing_map: serde_json::Map<String, serde_json::Value>) -> serde_json::Map<String, serde_json::Value> {
  if !existing_map.contains_key("title") {
    match get_title(json) {
      Some(title) => {
        existing_map.insert(String::from("title"), json!(title));
      },
      None => {}
    };
  }
  if !existing_map.contains_key("videoId") {
    match get_video_id(json) {
      Some(video_id) => {
        existing_map.insert(String::from("videoId"), json!(video_id));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("description") {
    match get_description(json) {
      Some(description) => {
        existing_map.insert(String::from("description"), json!(description));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("descriptionHtml") {
    match get_description_html(json) {
      Some(description_html) => {
        existing_map.insert(String::from("descriptionHtml"), json!(description_html));
      },
      None => {}
    };
  }
  if !existing_map.contains_key("published") {
    match get_published(json, lang) {
      Some(published) => {
        existing_map.insert(String::from("published"), json!(published));
      },
      None => match get_publish_date(json) {
        Some(published) => {
          existing_map.insert(String::from("published"), json!(published));
        },
        None => {}
      }
    };
  }
  if !existing_map.contains_key("publishedText") {
    match get_published_text(json) {
      Some(published_text) => {
        existing_map.insert(String::from("publishedText"), json!(published_text));
      },
      None => {}
    };
  }
  if !existing_map.contains_key("viewCount") {
    match get_view_count(json) {
      Some(view_count) => {
        existing_map.insert(String::from("viewCount"), json!(view_count));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("likeCount") {
    match get_like_count(json) {
      Some(like_count) => {
        existing_map.insert(String::from("likeCount"), json!(like_count));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("isFamilyFriendly") {
    match is_family_friendly(json) {
      Some(family_friendly) => {
        existing_map.insert(String::from("isFamilyFriendly"), json!(family_friendly));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("allowedRegions") {
    match get_available_countries(json) {
      Some(available_countries) => {
        existing_map.insert(String::from("allowedRegions"), json!(available_countries));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("allowedRegions") {
    match get_available_countries(json) {
      Some(available_countries) => {
        existing_map.insert(String::from("allowedRegions"), json!(available_countries));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("authorId") {
    match get_author_id(json) {
      Some(author_id) => {
        existing_map.insert(String::from("authorId"), json!(author_id));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("authorUrl") {
    match get_author_id(json) {
      Some(author_id) => {
        existing_map.insert(String::from("authorUrl"), json!(format!("/channel/{}", author_id)));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("authorThumbnails") {
    match get_author_thumbnails(json) {
      Some(author_thumbnails) => {
        existing_map.insert(String::from("authorThumbnails"), json!(author_thumbnails));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("keywords") {
    match get_keywords(json) {
      Some(keywords) => {
        existing_map.insert(String::from("keywords"), json!(keywords));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("viewCount") {
    match get_view_count(json) {
      Some(view_count) => {
        existing_map.insert(String::from("viewCount"), json!(view_count));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("likeCount") {
    match get_like_count(json) {
      Some(like_count) => {
        existing_map.insert(String::from("likeCount"), json!(like_count));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("isFamilyFriendly") {
    match is_family_friendly(json) {
      Some(family_friendly) => {
        existing_map.insert(String::from("isFamilyFriendly"), json!(family_friendly));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("allowedRegions") {
    match get_available_countries(json) {
      Some(available_countries) => {
        existing_map.insert(String::from("allowedRegions"), json!(available_countries));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("allowedRegions") {
    match get_available_countries(json) {
      Some(available_countries) => {
        existing_map.insert(String::from("allowedRegions"), json!(available_countries));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("genre") {
    match get_category(json) {
      Some(category) => {
        existing_map.insert(String::from("genre"), json!(category));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("author") {
    match get_author(json) {
      Some(author) => {
        existing_map.insert(String::from("author"), json!(author));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("authorId") {
    match get_author_id(json) {
      Some(author_id) => {
        existing_map.insert(String::from("authorId"), json!(author_id));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("authorUrl") {
    match get_author_id(json) {
      Some(author_id) => {
        existing_map.insert(String::from("authorUrl"), json!(format!("/channel/{}", author_id)));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("authorThumbnails") {
    match get_author_thumbnails(json) {
      Some(author_thumbnails) => {
        existing_map.insert(String::from("authorUrl"), json!(author_thumbnails));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("subCountText") {
    match get_sub_count_text(json) {
      Some(sub_count_text) => {
        existing_map.insert(String::from("subCountText"), json!(sub_count_text));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("lengthSeconds") {
    match get_length_seconds(json) {
      Some(length_seconds) => {
        existing_map.insert(String::from("lengthSeconds"), json!(length_seconds));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("allowRatings") {
    match get_allow_ratings(json) {
      Some(allow_ratings) => {
        existing_map.insert(String::from("allowRatings"), json!(allow_ratings));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("isListed") {
    match is_listed(json) {
      Some(is_listed) => {
        existing_map.insert(String::from("isListed"), json!(is_listed));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("liveNow") {
    match is_live_now(json) {
      Some(live_now) => {
        existing_map.insert(String::from("liveNow"), json!(live_now));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("hlsUrl") {
    match get_hls_url(json) {
      Some(hls_url) => {
        existing_map.insert(String::from("hlsUrl"), json!(hls_url));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("dashUrl") {
    match get_dash_url(json) {
      Some(dash_url) => {
        existing_map.insert(String::from("dashUrl"), json!(dash_url));
      },
      None => {}
    }
  }
  let decipher_code = create_formatable_decipher_js_code(player_res).unwrap_or(String::from(""));
  if !existing_map.contains_key("formatStreams") {
    match get_legacy_formats(json) {
      Some(legacy_formats) => {
        match create_formatable_decipher_js_code(player_res) {
          Ok(js_code) => {
            existing_map.insert(String::from("formatStreams"), legacy_formats.into_iter().map(|mut format| {
              match &format.signature_cipher {
                Some(cipher) => {
                  match format_decipher_code_into_executable(cipher, &js_code) {
                    Ok(executable_js_code) => {
                      match run_js_in_boa(executable_js_code) {
                        Ok(deciphered_url) => {
                          format.url = Some(add_host_param_to_url(&deciphered_url));
                        },
                        Err(_) => {}
                      }
                    },
                    Err(_) => {}
                  }
                },
                None => {
                  format.url = Some(add_host_param_to_url(&format.url.unwrap()));
                }
              };
              format.into_inv()
            }).collect::<Value>());
          },
          Err(_) => {}
        };
      },
      None => {}
    };
  }
  if !existing_map.contains_key("adaptiveFormats") {
    match get_adaptive_formats(json) {
      Some(mut formats) => {
        let mut formats_js_code = Vec::<String>::new();
        for i in 0..formats.len() {
          match &formats[i].signature_cipher {
            Some(cipher) => {
              match format_decipher_code_into_executable(cipher, &decipher_code) {
                Ok(js_code) => {
                  formats_js_code.push(String::from(js_code));
                },
                Err(_) => {}
              }
            },
            None => {}
          }
        }
        if formats_js_code.len() > 0 {
          let js_code = format!("[(() => {{{}}})()]", formats_js_code.join("})(),(() => {")).replace("deciphered_url}", "return deciphered_url}");
          match run_js_in_boa(js_code) {
            Ok(deciphered_urls) => {
              let urls = deciphered_urls.split(",").collect::<Vec::<&str>>();
              for i in 0..formats.len() {
                formats[i].url = Some(add_host_param_to_url(urls[i]));
              }
            },
            Err(_) => {}
          }
        } else {
          for i in 0..formats.len() {
            formats[i].url = Some(add_host_param_to_url(&formats[i].url.as_ref().unwrap()));
          }
        }
        existing_map.insert(String::from("adaptiveFormats"), formats.into_iter().map(|format| format.into_inv()).collect::<Value>());
      },
      None => {}
    }
  }
  if !existing_map.contains_key("captions") {
    match get_captions(json) {
      Some(captions) => {
        existing_map.insert(String::from("captions"), json!(captions));
      },
      None => {}
    }
  }
  if !existing_map.contains_key("recommendedVideos") {
    match get_recommended(json) {
      Some(videos) => {
        existing_map.insert(String::from("recommendedVideos"), videos.into_iter().filter_map(|format| format.into_inv()).collect::<Value>());
      },
      None => {}
    }
  }
  if !existing_map.contains_key("musicTracks") {
    match get_music_tracks(json) {
      Some(tracks) => {
        existing_map.insert(String::from("musicTracks"), tracks.into_iter().map(|track| track.into_inv()).collect::<Value>());
      },
      None => {}
    }
  }
  existing_map
}

// Gets the title from the `/next` or `/player` endpoint response
pub fn get_title(json: &Value) -> Option<String> {
  match 
  json["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][0]["videoPrimaryInfoRenderer"]["title"]["runs"][0]["text"].as_str() {
    Some(title) => Some(String::from(title)),
    None => match json["videoDetails"]["title"].as_str() {
      Some(title) => Some(String::from(title)),
      None => None
    }
  }
}

pub fn get_video_id(json: &Value) -> Option<&str> {
  json["currentVideoEndpoint"]["watchEndpoint"]["videoId"].as_str()
}

// Gets the description from the `/next` or `/player` endpoint response
pub fn get_description(json: &Value) -> Option<String> {
  match json["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][1]["videoSecondaryInfoRenderer"]["attributedDescription"]["content"].as_str() {
    Some(description) => Some(String::from(description)),
    None => match json["videoDetails"]["shortDescription"].as_str() {
      Some(description) => Some(String::from(description)),
      None => None
    }
  }
}

// Gets the description html from the `/next` endpoint response 
pub fn get_description_html(json: &Value) -> Option<String> {
  match json["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][1]["videoSecondaryInfoRenderer"]["attributedDescription"]["content"].as_str() {
    Some(attributed_description) => {
      match json["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][1]["videoSecondaryInfoRenderer"]["attributedDescription"]["commandRuns"].as_array() {
        Some(command_runs) => {
          if command_runs.len() > 0 {
            let mut offset: usize = 0;
            let encoded_attributed_description = attributed_description.encode_utf16().collect::<Vec::<u16>>();
            let mut description_with_attr = encoded_attributed_description.clone();
            for i in 0..command_runs.len() {
              let command_run = &command_runs[i];
              let start_index = match command_run["startIndex"].as_i64() {
                Some(start_index) => start_index as usize,
                None => 0
              };
              let length = match command_run["length"].as_i64() {
                Some(length) => length as usize,
                None => 0
              };
              if length != 0 {
                let url = match command_run["onTap"]["innertubeCommand"]["urlEndpoint"]["url"].as_str() {
                  Some(url_endpoint) => {
                    if url_endpoint.contains("&q=") && url_endpoint.contains("&v=") {
                      Some(String::from(decode(url_endpoint.split("&q=").collect::<Vec::<&str>>()[1].to_string().split("&v=").collect::<Vec::<&str>>()[0]).unwrap()))
                    } else if !url_endpoint.contains("https://www.youtube.com/redirect") {
                      Some(String::from(url_endpoint))
                    } else {
                      None
                    }
                  },
                  None => match command_run["onTap"]["innertubeCommand"]["watchEndpoint"]["videoId"].as_str() {
                    Some(video_id) => {
                      match command_run["onTap"]["innertubeCommand"]["watchEndpoint"]["startTimeSeconds"].as_i64() {
                        Some(start_time_seconds) => {
                          if start_time_seconds > 0 {
                            Some(String::from(format!("{}/{}?t={}", SHORT_WEBSITE_BASE_URL, video_id, start_time_seconds)))
                          } else {
                            Some(String::from(format!("{}/{}", SHORT_WEBSITE_BASE_URL, video_id)))
                          }
                        },
                        None => None
                      }
                    },
                    None => match command_run["onTap"]["innertubeCommand"]["browseEndpoint"]["canonicalBaseUrl"].as_str() {
                      Some(base_url) => {
                        Some(String::from(format!("{}{}", WEBSITE_BASE_URL, base_url)))
                      },
                      None => None
                    }
                  }
                };
                match url {
                  Some(url) => {
                    let link_contents = match encoded_attributed_description.subvector_as_str(start_index, start_index + length) {
                      Ok(contents) => contents,
                      Err(err) => {
                        println!("ERROR when decoding description html link contents: {}", err.message);
                        String::from("")
                      }
                    };

                    let attr_len = encoded_attributed_description.len();
                    let before_author_name = Regex::new(r"/\s@*").unwrap();
                    let link = format!("<a href=\"{}\">{}</a>" , url, before_author_name.replace(&link_contents.replace("•", ""), "@").trim());
                    let before = match description_with_attr.subvector_as_str(0, start_index + offset) {
                      Ok(contents) => contents,
                      Err(err) => {
                        println!("ERROR when decoding description html: {}", err.message);
                        String::from("")
                      }
                    };

                    let after = match description_with_attr.subvector_as_str(start_index + length + offset, description_with_attr.len()) {
                      Ok(contents) => contents,
                      Err(err) => {
                        println!("ERROR when decoding description html: {}", err.message);
                        String::from("")
                      }
                    };
                    
                    description_with_attr = String::from(format!("{}{}{}", before, link, after)).encode_utf16().collect::<Vec::<u16>>();
                    
                    offset = description_with_attr.len() - attr_len;
                  },
                  None => {}
                };
              }
            }
            Some(String::from_utf16(&description_with_attr).unwrap())
          } else {
            Some(String::from(attributed_description))
          }
        },
        None => Some(String::from(attributed_description))
      }
    }
    None => None
  }
}

// Gets the full date text from the `/next` endpoint response
pub fn get_date_text(json: &Value) -> Option<String> {
  match json["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][0]["videoPrimaryInfoRenderer"]["dateText"]["simpleText"].as_str() {
    Some(date_text) => {
      Some(String::from(date_text))
    },
    None => None
  }
}

// Gets the month+day and the year the video was uploaded from the `/next` endpoint response
pub fn get_date_parts(json: &Value) -> Option<(String, String)> {
  match json["engagementPanels"].as_array() {
    Some(panels) => {
      let mut final_result = None;
      for i in 0..panels.len() {
        let result = match panels[i]["engagementPanelSectionListRenderer"]["content"]["structuredDescriptionContentRenderer"]["items"][0]["videoDescriptionHeaderRenderer"]["factoid"][2]["factoidRenderer"]["value"]["simpleText"].as_str() {
          Some(value) => {
            match json["engagementPanels"][i]["engagementPanelSectionListRenderer"]["content"]["structuredDescriptionContentRenderer"]["items"][0]["videoDescriptionHeaderRenderer"]["factoid"][2]["factoidRenderer"]["label"]["simpleText"].as_str() {
              Some(label) => {
                if label.contains(" ") {
                  Some((String::from(label), String::from(value)))
                } else {
                  Some((String::from(value), String::from(label)))
                }
              },
              None => None
            }
          },
          None => None
        };
        match result {
          Some(_) => {
            final_result = result;
            break
          },
          None => {}
        };
      }
      final_result
    },
    None => None
  }
}

// Gets the published timestamp from the `/next` endpoint response
#[cfg(feature = "parse_languages_to_published")]
pub fn get_published(json: &Value, lang: &str) -> Option<i64> {
  // let's just say we can assume that this code will not be functional in 100 years anyway
  let match_years = Regex::new(r"20[0-9][0-9]").unwrap();
  let month_map = match serde_json::from_str::<Value>(include_str!("../../../data/language-month-map.json")) {
    Ok(map) => {
      match map[lang].as_array() {
        Some(months) => {
          Some(months.clone())
        },
        None => {
          println!("WARN: language pref not found");
          println!("defaulting to english");
          if lang != "en" {
            match map["en"].as_array() {
              Some(months) => Some(months.clone()),
              None => None
            }
          } else {
            None
          }
        }
      }
    },
    Err(error) => {
      println!("ERROR: {}", error);
      None
    }
  };
  let parts = match get_date_parts(json) {
    Some(parts) => Some(parts),
    None => {
      match get_date_text(json) {
        Some(date_text) => {
          match match_years.captures(&date_text) {
            Some(year_matches) => {
              Some((String::from(year_matches.get(0).unwrap().as_str()), String::from(match_years.replace(&date_text, ""))))
            },
            None => None
          }
        },
        None => None
      }
    }
  };
  match parts {
    Some((label, value)) => {
      let year = match match_years.captures(&label) {
        Some(_) => String::from(&label),
        None => String::from(&value)
      };
      let year_num = match match_years.captures(&year) {
        Some(year_num_string) => {
          match i32::from_str(&year_num_string[0]) {
            Ok(year_num) => Some(year_num),
            Err(_) => None
          }
        },
        None => None
      };

      let day_month = if year == label {
        String::from(&value)
      } else {
        String::from(&label)
      };
      match month_map {
        Some(in_order_months) => {
          let months = &in_order_months.into_iter().rev().collect::<Vec::<Value>>();
          let mut month = 12;
          let mut day = 1;
          for i in 0..months.len() {
            let month_name = months[i].as_str().unwrap();
            if day_month.contains(month_name) {
              month = months.len() - i;// add one because months are 1 indexed in dates
              let date_without_month = day_month.replace(month_name, "");
              let some_number = Regex::new(r"[0-9]+").unwrap();
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
          match year_num {
            Some(year_safe) => {
              let date_string = format!("{}-{}-{}", year_safe, month, day);
              match NaiveDate::parse_from_str(&date_string, "%Y-%m-%d") {
                Ok(date_time) => {
                  Some(date_time.and_time(NaiveTime::default()).timestamp())
                },
                Err(_) => None
              }
            },
            None => None
          }
        },
        None => {
          None
        }
      }
    },
    None => None
  }
}

// Gets the published date (as a timestamp) from the `/player` endpoint response
pub fn get_publish_date(json: &Value) -> Option<i64> {
  match json["microformat"]["playerMicroformatRenderer"]["publishDate"].as_str() {
    Some(published_date_string) => {
      match NaiveDate::parse_from_str(&published_date_string, "%Y-%m-%d") {
        Ok(published) => Some(published.and_time(NaiveTime::default()).timestamp()),
        Err(_) => None
      }
    },
    None => None
  }
}

// Gets the published text from the `/next` endpoint response
pub fn get_published_text(json: &Value) -> Option<String> {
  match json["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][0]["videoPrimaryInfoRenderer"]["relativeDateText"]["simpleText"].as_str() {
    Some(date_text) => {
      Some(String::from(date_text))
    },
    None => None
  }
}

// Gets the keywords from the `/player` endpoint response
pub fn get_keywords(json: &Value) -> Option<Vec::<String>> {
  match json["videoDetails"]["keywords"].as_array() {
    Some(keyword_array) => Some(keyword_array.into_iter().filter_map(|keyword| {
      match keyword.as_str() {
        Some(keyword) => Some(String::from(keyword)),
        None => None
      }
    }).collect::<Vec::<String>>()),
    None => None
  }
}

// Gets the view count from the `/next` or the `/player` endpoint response
pub fn get_view_count(json: &Value) -> Option<i32> {
  let match_numbers = Regex::new(r"[0-9]+").unwrap();
  let views_text = match json["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][0]["videoPrimaryInfoRenderer"]["viewCount"]["videoViewCountRenderer"]["viewCount"]["simpleText"].as_str() {
    Some(views_text) => Some(views_text),
    None => match json["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][0]["videoPrimaryInfoRenderer"]["viewCount"]["videoViewCountRenderer"]["viewCount"]["runs"][0]["text"].as_str() {
      Some(views_text) => Some(views_text),
      None => None
    }
  };
  match views_text {
    Some(view_text) => {
      let view_numbers_string = match_numbers.find_iter(view_text).map(|m| m.as_str()).collect::<String>();
      match i32::from_str(&view_numbers_string) {
        Ok(number) => Some(number),
        Err(_) => match json["engagementPanels"].as_array() {
          Some(panels) => {
            let mut final_result = None;
            for i in 0..panels.len() {
              final_result = match panels[i]["engagementPanelSectionListRenderer"]["content"]["structuredDescriptionContentRenderer"]["items"][0]["videoDescriptionHeaderRenderer"]["factoid"][1]["factoidRenderer"]["value"]["simpleText"].as_str() {
                Some(views_factoid) => { 
                  if views_factoid == "0" {
                    Some(0)
                  } else {
                    None
                  }
                },
                None => None
              };
              match final_result {
                Some(_) => { break },
                None => {}
              };
            }
            final_result
          },
          None => None
        }
      }
    },
    None => match json["videoDetails"]["viewCount"].as_str() {
      Some(view_count) => {
        match i32::from_str(view_count) {
          Ok(views) => Some(views),
          Err(_) => None
        }
      },
      None => None
    }
  }
}

// Gets the like count from the `/next` endpoint response
pub fn get_like_count(json: &Value) -> Option<i32> {
  let match_numbers = Regex::new(r"[0-9]+").unwrap();
  match json["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][0]["videoPrimaryInfoRenderer"]["videoActions"]["menuRenderer"]["topLevelButtons"][0]["segmentedLikeDislikeButtonRenderer"]["likeButton"]["toggleButtonRenderer"]["defaultText"]["accessibility"]["accessibilityData"]["label"].as_str() {
    Some(like_text) => {
      let like_numbers_string = match_numbers.find_iter(like_text).map(|m| m.as_str()).collect::<String>();
      match i32::from_str(&like_numbers_string) {
        Ok(number) => Some(number),
        Err(_) => match json["engagementPanels"].as_array() {
          Some(panels) => {
            let mut final_result = None;
            for i in 0..panels.len() {
              final_result = match panels[i]["engagementPanelSectionListRenderer"]["content"]["structuredDescriptionContentRenderer"]["items"][0]["videoDescriptionHeaderRenderer"]["factoid"][0]["factoidRenderer"]["value"]["simpleText"].as_str() {
                Some(views_factoid) => { 
                  if views_factoid == "0" {
                    Some(0)
                  } else {
                    None
                  }
                },
                None => None
              };
              match final_result {
                Some(_) => { break },
                None => {}
              };
            }
            final_result
          },
          None => None
        }
      }
    },
    None => None
  }
}

// Gets the author from the `/next` or the `/player` endpoint response
pub fn get_author(json: &Value) -> Option<String> {
  match json["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][1]["videoSecondaryInfoRenderer"]["owner"]["videoOwnerRenderer"]["title"]["runs"][0]["text"].as_str() {
    Some(owner) => Some(String::from(owner)),
    None => match json["videoDetails"]["author"].as_str() {
      Some(author) => Some(String::from(author)),
      None => None
    }
  }
}

// Gets the author id from the `/next` or the `/player` endpoint response
pub fn get_author_id(json: &Value) -> Option<String> {
  match json["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][1]["videoSecondaryInfoRenderer"]["owner"]["videoOwnerRenderer"]["title"]["runs"][0]["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str() {
    Some(browse_id) => Some(String::from(browse_id)),
    None => match json["videoDetails"]["channelId"].as_str() {
      Some(channel_id) => Some(String::from(channel_id)),
      None => None
    }
  }
}

#[derive(Deserialize, Serialize)]
pub struct Thumbnail {
  pub url: String,
  pub width: i32,
  pub height: i32
}

pub fn get_author_thumbnails(json: &Value) -> Option<Vec<Thumbnail>> {
  match json["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][1]["videoSecondaryInfoRenderer"]["owner"]["videoOwnerRenderer"]["thumbnail"]["thumbnails"].as_array() {
    Some(author_thumbnails) => {
      Some(author_thumbnails.into_iter().map(|thumbnail| {
        Thumbnail {
          url: String::from(thumbnail["url"].as_str().unwrap_or("")),
          width: thumbnail["width"].as_i64().unwrap_or(0) as i32,
          height: thumbnail["height"].as_i64().unwrap_or(0) as i32
        }
      }).collect::<Vec<Thumbnail>>())
    },
    None => None
  }
}

// Gets the sub count text from the `/next` endpoint response
pub fn get_sub_count_text(json: &Value) -> Option<String> {
  let match_numbers = Regex::new(r"[0-9]+").unwrap();
  match json["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][1]["videoSecondaryInfoRenderer"]["owner"]["videoOwnerRenderer"]["subscriberCountText"]["simpleText"].as_str() {
    Some(text) => Some(String::from(text.split(" ").filter_map(|part| { 
      match match_numbers.captures(part) {
        Some(_) => {
          Some(part)
        },
        None => None
      }
    }).collect::<Vec::<&str>>()[0])),
    None => None
  }
}

pub fn get_length_seconds(json: &Value) -> Option<i32> {
  match json["videoDetails"]["lengthSeconds"].as_str() {
    Some(length_seconds_str) => {
      match i32::from_str(length_seconds_str) {
        Ok(length_seconds) => Some(length_seconds),
        Err(_) => None
      }
    },
    None => None
  }
}

pub fn get_allow_ratings(json: &Value) -> Option<bool> {
  json["videoDetails"]["allowRatings"].as_bool()
}

// Gets the listed status from the `/next` or the `/player` endpoint response
pub fn is_listed(json: &Value) -> Option<bool> {
  match json["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][0]["videoPrimaryInfoRenderer"]["badges"].as_array() {
    Some(badges) => {
      let mut result = false;
      for i in 0..badges.len() {
        result = result || match badges[i]["metadataBadgeRenderer"]["icon"]["iconType"].as_str() {
          Some(label) => label != "PRIVACY_UNLISTED",
          None => true
        };
        if result {
          break;
        }
      }
      Some(result)
    },
    None => {
      match json["videoDetails"]["isCrawlable"].as_bool() {
        Some(is_listed) => Some(is_listed),
        None => None
      }
    }
  }
}

// Gets the `formats` from the `/next` or the `/player` endpoint response
pub fn is_live_now(json: &Value) -> Option<bool> {
  match json["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][0]["videoPrimaryInfoRenderer"]["viewCount"]["videoViewCountRenderer"]["isLive"].as_bool() {
    Some(is_live) => Some(is_live),
    None => {
      match json["videoDetails"]["isLiveContent"].as_bool() {
        Some(is_live) => Some(is_live),
        None => None
      }
    }
  }
}

pub fn get_dash_url(json: &Value) -> Option<String> {
  match json["streamingData"]["dashManifestUrl"].as_str() {
    Some(dash_url) => Some(String::from(dash_url)),
    None => None
  }
}

pub fn get_hls_url(json: &Value) -> Option<String> {
  match json["streamingData"]["hlsManifestUrl"].as_str() {
    Some(hls_url) => Some(String::from(hls_url)),
    None => None
  }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct LegacyFormat {
  pub url: Option<String>,
  pub signature_cipher: Option<String>,
  pub itag: i32,
  pub mime_type: String,
  pub bitrate: i32,
  pub width: i32,
  pub height: i32,
  pub last_modified_time: i64,
  pub quality: String,
  pub fps: i32,
  pub quality_label: String,
  pub projection_type: String,
  pub audio_quality: String,
  pub approx_duration_ms: i32,
  pub audio_sample_rate: String,
  pub audio_channels: i32
}

impl LegacyFormat {
  pub fn into_inv(&self) -> serde_json::Map<String, Value> {
    let mut j_object = serde_json::Map::new();
    match &self.url {
      Some(url) => {
        j_object.insert(String::from("url"), json!(url));
      },
      None => {}
    };
    j_object.insert(String::from("itag"), json!(format!("{}", self.itag)));
    j_object.insert(String::from("type"), json!(self.mime_type));
    j_object.insert(String::from("quality"), json!(self.quality));
    j_object.insert(String::from("fps"), json!(self.fps));
    let container_re = Regex::new(r"(video|audio)/([^;]*?); ").unwrap();
    match container_re.captures(&self.mime_type) {
      Some(container) => {
        j_object.insert(String::from("container"), json!(container.get(2).unwrap().as_str()));
      },
      None => {}
    };
    j_object.insert(String::from("resolution"), json!(self.quality_label));
    j_object.insert(String::from("qualityLabel"), json!(self.quality_label));
    j_object.insert(String::from("size"), json!(format!("{}x{}", self.width, self.height)));
    j_object
  }
}

// Gets the `formats` from the `/player` endpoint response
pub fn get_legacy_formats(json: &Value) -> Option<Vec<LegacyFormat>> {
  match json["streamingData"]["formats"].as_array() {
    Some(formats) => {
      Some(formats.into_iter().map(|format| {
        LegacyFormat {
          url: match format["url"].as_str() {
            Some(url) => Some(String::from(url)),
            None => None
          },
          signature_cipher: match format["signatureCipher"].as_str() {
            Some(cipher) => Some(String::from(cipher)),
            None => None
          },
          itag: format["itag"].as_i64().unwrap_or(0) as i32,
          mime_type: String::from(format["mimeType"].as_str().unwrap_or("")),
          bitrate: format["bitrate"].as_i64().unwrap_or(0) as i32,
          width: format["width"].as_i64().unwrap_or(0) as i32,
          height: format["height"].as_i64().unwrap_or(0) as i32,
          last_modified_time: i64::from_str(format["lastModified"].as_str().unwrap_or("")).unwrap_or(0),
          quality: String::from(format["quality"].as_str().unwrap_or("")),
          fps: format["fps"].as_i64().unwrap_or(0) as i32,
          quality_label: String::from(format["qualityLabel"].as_str().unwrap_or("")),
          projection_type: String::from(format["projectionType"].as_str().unwrap_or("")),
          audio_quality: String::from(format["audioQuality"].as_str().unwrap_or("")),
          approx_duration_ms: i32::from_str(format["approxDurationMs"].as_str().unwrap_or("")).unwrap_or(0),
          audio_sample_rate: String::from(format["audioSampleRate"].as_str().unwrap_or("")),
          audio_channels: format["audioChannels"].as_i64().unwrap_or(0) as i32
        }
      }).collect())
    },
    None => None
  }
}

pub fn is_family_friendly(json: &Value) -> Option<bool> {
  json["microformat"]["playerMicroformatRenderer"]["isFamilySafe"].as_bool()
}

// Gets the `availableCountries` from the `/player` endpoint response
pub fn get_available_countries(json: &Value) -> Option<Vec<&str>> {
  match json["microformat"]["playerMicroformatRenderer"]["availableCountries"].as_array() {
    Some(available_countries) => {
      Some(available_countries.into_iter().filter_map(|country| country.as_str()).collect::<Vec::<&str>>())
    },
    None => None
  }
}

// Gets the `category` from the `/player` endpoint response
pub fn get_category(json: &Value) -> Option<String> {
  match json["microformat"]["playerMicroformatRenderer"]["category"].as_str() {
    Some(category) => Some(String::from(category)),
    None => None
  }
}

pub struct AdaptiveFormat {
  pub url: Option<String>,
  pub signature_cipher: Option<String>,
  pub itag: i32,
  pub mime_type: String,
  pub bitrate: i32,
  pub width: Option<i32>,
  pub height: Option<i32>,
  pub init_range: (i32, i32),
  pub index_range: (i32, i32),
  pub last_modified_time: i64,
  pub content_length: i32,
  pub quality: String,
  pub fps: Option<i32>,
  pub quality_label: Option<String>,
  pub projection_type: String,
  pub average_bitrate: i32,
  pub approx_duration_ms: i32,
  pub audio_sample_rate: Option<i32>,
  pub audio_quality: Option<String>,
  pub loudness_db: Option<f32>,
  pub audio_channels: Option<i32>
}

impl AdaptiveFormat {
  pub fn into_inv(&self) -> serde_json::Map<String, Value> {
    let mut j_object = serde_json::Map::new();
    match &self.url {
      Some(url) => {
        j_object.insert(String::from("url"), json!(url));
      },
      None => {}
    };
    j_object.insert(String::from("bitrate"), json!(format!("{}", self.bitrate)));
    j_object.insert(String::from("init"), json!(format!("{}-{}", self.init_range.0, self.init_range.1)));
    j_object.insert(String::from("index"), json!(format!("{}-{}", self.index_range.0, self.index_range.1)));
    j_object.insert(String::from("itag"), json!(format!("{}", self.itag)));
    j_object.insert(String::from("type"), json!(self.mime_type));
    j_object.insert(String::from("clen"), json!(format!("{}", self.content_length)));
    j_object.insert(String::from("lmt"), json!(self.last_modified_time));
    j_object.insert(String::from("projectionType"), json!(self.projection_type));
    match self.fps {
      Some(fps) => {
        j_object.insert(String::from("fps"), json!(fps));
      },
      None => {}
    };
    let container_re = Regex::new(r"(video|audio)/([^;]*?); ").unwrap();
    match container_re.captures(&self.mime_type) {
      Some(container) => {
        j_object.insert(String::from("container"), json!(container.get(2).unwrap().as_str()));
      },
      None => {}
    };
    match &self.audio_quality {
      Some(audio_quality) => {
        j_object.insert(String::from("audioQuality"), json!(audio_quality));
      },
      None => {}
    };
    match self.audio_sample_rate {
      Some(audio_sample_rate) => {
        j_object.insert(String::from("audioSampleRate"), json!(audio_sample_rate));
      },
      None => {}
    };
    match self.audio_channels {
      Some(audio_channels) => {
        j_object.insert(String::from("audioChannels"), json!(audio_channels));
      },
      None => {}
    }
    j_object.insert(String::from("quality"), json!(self.quality));
    match &self.quality_label {
      Some(quality_label) => {
        j_object.insert(String::from("qualityLabel"), json!(quality_label));
      },
      None => {
        j_object.insert(String::from("qualityLabel"), json!(format!("{}", self.bitrate)));
      }
    };    
    match (self.width, self.height) {
      (Some(width), Some(height)) => {
        j_object.insert(String::from("size"), json!(format!("{}x{}", width, height)));
      },
      _ => {}
    }
    j_object
  }
}

pub fn get_adaptive_formats(json: &Value) -> Option<Vec<AdaptiveFormat>> {
  match json["streamingData"]["adaptiveFormats"].as_array() {
    Some(adaptive_formats) => {
      Some(adaptive_formats.into_iter().map(|format| {
        AdaptiveFormat {
          url: match format["url"].as_str() {
            Some(url) => Some(String::from(url)),
            None => None
          },
          signature_cipher: match format["signatureCipher"].as_str() {
            Some(signature_cipher) => Some(String::from(signature_cipher)),
            None => None
          },
          itag: format["itag"].as_i64().unwrap_or(0) as i32,
          mime_type: String::from(format["mimeType"].as_str().unwrap_or("")),
          bitrate: format["bitrate"].as_i64().unwrap_or(0) as i32,
          width: match format["width"].as_i64() {
            Some(width) => Some(width as i32),
            None => None
          },
          height: match format["height"].as_i64() {
            Some(height) => Some(height as i32),
            None => None
          },
          init_range: (i32::from_str(format["initRange"]["start"].as_str().unwrap_or("")).unwrap_or(0), i32::from_str(format["initRange"]["end"].as_str().unwrap_or("")).unwrap_or(0)),
          index_range: (i32::from_str(format["indexRange"]["start"].as_str().unwrap_or("")).unwrap_or(0), i32::from_str(format["indexRange"]["end"].as_str().unwrap_or("")).unwrap_or(0)),
          last_modified_time: i64::from_str(format["lastModified"].as_str().unwrap_or("")).unwrap_or(0),
          content_length: i32::from_str(format["contentLength"].as_str().unwrap_or("")).unwrap_or(0),
          quality: String::from(format["quality"].as_str().unwrap_or("")),
          fps: match format["fps"].as_i64() {
            Some(fps) => Some(fps as i32),
            None => None
          },
          quality_label: match format["qualityLabel"].as_str() {
            Some(quality_label) => Some(String::from(quality_label)),
            None => None
          },
          projection_type: String::from(format["projectionType"].as_str().unwrap_or("")),
          average_bitrate: format["averageBitrate"].as_i64().unwrap_or(0) as i32,
          approx_duration_ms: i32::from_str(format["approxDurationMs"].as_str().unwrap_or("")).unwrap_or(0),
          audio_sample_rate: match format["audioSampleRate"].as_str() {
            Some(audio_sample_rate) => {
              match i32::from_str(audio_sample_rate) {
                Ok(audio_sample_rate) => Some(audio_sample_rate),
                Err(_) => None
              }
            },
            None => None
          },
          audio_quality: match format["audioQuality"].as_str() {
            Some(audio_quality) => {
              Some(String::from(audio_quality))
            },
            None => None
          },
          loudness_db: match format["loudnessDb"].as_f64() {
            Some(loudness_db) => Some(loudness_db as f32),
            None => None
          },
          audio_channels: match format["audioChannels"].as_i64() {
            Some(audio_channels) => Some(audio_channels as i32),
            None => None
          }
        }
      }).collect::<Vec<AdaptiveFormat>>())
    },
    None => None
  }
}

#[derive(Deserialize, Serialize)]
pub struct Chapter {
  pub title: String,
  pub time_range_start_millis: i32,
  pub thumbnails: Vec<Thumbnail>
}

pub fn get_chapters(json: &Value) -> Option<Vec<Chapter>> {
  match json["playerOverlays"]["playerOverlayRenderer"]["decoratedPlayerBarRenderer"]["decoratedPlayerBarRenderer"]["playerBar"]["multiMarkersPlayerBarRenderer"]["markersMap"][0]["value"]["chapters"].as_array() {
    Some(chapters) => {
      Some(chapters.into_iter().map(|chapter| {
        Chapter {
          title: String::from(chapter["chapterRenderer"]["title"]["simpleText"].as_str().unwrap_or("")),
          time_range_start_millis: chapter["chapterRenderer"]["timeRangeStartMillis"].as_i64().unwrap_or(0) as i32,
          thumbnails: chapter["chapterRenderer"]["thumbnail"]["thumbnails"].as_array().unwrap_or(&Vec::<Value>::new()).into_iter().map(|thumbnail| {
            Thumbnail {
              url: String::from(thumbnail["url"].as_str().unwrap_or("")),
              width: thumbnail["width"].as_i64().unwrap_or(0) as i32,
              height: thumbnail["height"].as_i64().unwrap_or(0) as i32
            }
          }).collect::<Vec::<Thumbnail>>()
        }
      }).collect::<Vec::<Chapter>>())
    },
    None => None
  }
}

#[derive(Deserialize, Serialize)]
pub struct Caption {
  pub label: String,
  pub language_code: String,
  pub url: String
}

pub fn get_captions(json: &Value) -> Option<Vec::<Caption>> {
  match json["captions"]["playerCaptionsTracklistRenderer"]["captionTracks"].as_array() {
    Some(caption_tracks) => {
      Some(caption_tracks.into_iter().map(|track| {
        Caption {
          label: String::from(track["name"]["simpleText"].as_str().unwrap_or("")),
          language_code: String::from(track["languageCode"].as_str().unwrap_or("")),
          // set the url to the correct return format
          url: track["baseUrl"].as_str().unwrap_or("").replace("&ip=0.0.0.0", "&ip=0.0.0.0&fmt=vtt")
        }
      }).collect::<Vec::<Caption>>())
    },
    None => None 
  }
}

#[derive(Deserialize, Serialize)]
pub struct RecommendedItem {
  pub video_id: Option<String>,
  pub thumbnails: Vec::<Thumbnail>,
  pub title: Option<String>,
  pub author: Option<String>,
  pub author_id: Option<String>,
  pub published_time_text: Option<String>,
  pub view_count: Option<i32>,
  pub length_seconds: Option<i32>,
  pub playlist_id: Option<String>,
  pub video_count: Option<String>
}

impl RecommendedItem {
  pub fn into_inv(&self) -> Option<serde_json::Map<String, Value>> {
    let mut j_object = serde_json::Map::new();
    match (&self.video_id, &self.title, &self.author, &self.author_id, self.length_seconds, self.view_count) {
      (Some(video_id), Some(title), Some(author), Some(author_id), Some(length_seconds), Some(view_count)) => {
        j_object.insert(String::from("videoId"), json!(video_id));
        j_object.insert(String::from("title"), json!(title));
        j_object.insert(String::from("videoThumbnails"), json!(self.thumbnails));
        j_object.insert(String::from("author"), json!(author));
        j_object.insert(String::from("authorUrl"),  json!(format!("/channel/{}", author_id)));
        j_object.insert(String::from("authorId"), json!(author_id));
        j_object.insert(String::from("lengthSeconds"), json!(length_seconds));
        j_object.insert(String::from("viewCount"), json!(view_count));
        Some(j_object)
      },
      _ => {
        None
      }
    }
  }
}

pub fn get_recommended(json: &Value) -> Option<Vec::<RecommendedItem>>  {
  let match_numbers = Regex::new(r"[0-9]+").unwrap();
  match json["contents"]["twoColumnWatchNextResults"]["secondaryResults"]["secondaryResults"]["results"].as_array() {
    Some(recommended_videos) => {
      Some(recommended_videos.into_iter().map(|recommended_video| {
        RecommendedItem {
          video_id: match recommended_video["compactVideoRenderer"]["videoId"].as_str() {
            Some(video_id) => Some(String::from(video_id)),
            None => None
          },
          thumbnails: recommended_video["compactVideoRenderer"]["thumbnail"]["thumbnails"].as_array().unwrap_or(&Vec::<Value>::new()).into_iter().map(|thumbnail| {
            Thumbnail {
              url: String::from(thumbnail["url"].as_str().unwrap_or("")),
              width: thumbnail["width"].as_i64().unwrap_or(0) as i32,
              height: thumbnail["height"].as_i64().unwrap_or(0) as i32
            }
          }).collect::<Vec::<Thumbnail>>(),
          title: match recommended_video["compactVideoRenderer"]["title"]["simpleText"].as_str() {
            Some(title) => Some(String::from(title)),
            None => match recommended_video["compactRadioRenderer"]["title"]["simpleText"].as_str() {
              Some(title) => Some(String::from(title)),
              None => None
            }
          },
          author: match recommended_video["compactVideoRenderer"]["longBylineText"]["runs"][0]["text"].as_str() {
            Some(author) => Some(String::from(author)),
            None => None
          },
          author_id: match recommended_video["compactVideoRenderer"]["longBylineText"]["runs"][0]["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str() {
            Some(author_id) => Some(String::from(author_id)),
            None => None
          },
          published_time_text: match recommended_video["compactVideoRenderer"]["publishedTimeText"]["simpleText"].as_str() {
            Some(published_time_text) => Some(String::from(published_time_text)),
            None => None
          },
          view_count: match recommended_video["compactVideoRenderer"]["viewCountText"]["simpleText"].as_str() {
            Some(view_count_text) => {
              match i32::from_str(&match_numbers.find_iter(view_count_text).map(|number| number.as_str()).collect::<String>()) {
                Ok(view_count) => Some(view_count),
                Err(_) => None
              }
            },
            None => None
          },
          length_seconds: match recommended_video["compactVideoRenderer"]["lengthText"]["simpleText"].as_str() {
            Some(length_string) => {
              let mut numbers = match_numbers.find_iter(length_string).map(|capture| capture.as_str()).collect::<Vec::<&str>>();
              numbers.reverse();
              let mut length = 0;
              let mut magnitude = 1;
              let multiplier = 60;
              for i in 0..numbers.len() {
                length = length + i32::from_str(numbers[i]).unwrap_or(0) * magnitude;
                magnitude = magnitude * multiplier;
              }
              Some(length)
            },
            None => None
          },
          playlist_id: match recommended_video["compactRadioRenderer"]["playlistId"].as_str() {
            Some(playlist_id) => Some(String::from(playlist_id)),
            None => None
          },
          video_count: match recommended_video["compactRadioRenderer"]["videoCountText"]["runs"][0]["text"].as_str() {
            Some(video_count) => Some(String::from(video_count)),
            None => None
          }
        }
      }).collect::<Vec::<RecommendedItem>>())
    },
    None => None
  }
}

#[derive(Deserialize, Serialize)]
pub struct MusicTrack {
  song: Option<String>,
  artist: Option<String>,
  album: Option<String>,
  license: Option<String>,
  video_id: Option<String>
}

impl MusicTrack {
  fn into_inv(&self) -> serde_json::Map::<String, Value> {
    let mut j_object = serde_json::Map::<String, Value>::new();
    match &self.song {
      Some(song) => {
        j_object.insert(String::from("song"), json!(song));
      },
      None => {}
    };
    match &self.artist {
      Some(artist) => {
        j_object.insert(String::from("artist"), json!(artist));
      },
      None => {}
    };
    match &self.album {
      Some(album) => {
        j_object.insert(String::from("album"), json!(album));
      },
      None => {}
    };
    match &self.license {
      Some(license) => {
        j_object.insert(String::from("license"), json!(license));
      },
      None => {}
    };
    match &self.video_id {
      Some(video_id) => {
        j_object.insert(String::from("videoId"), json!(video_id));
      },
      None => {}
    };
    j_object
  }
}

pub fn get_music_tracks(json: &Value) -> Option<Vec::<MusicTrack>> {
  match json["engagementPanels"].as_array() {
    Some(panels) => {
      for i in 0..panels.len() {
        let result = match panels[i]["engagementPanelSectionListRenderer"]["content"]["structuredDescriptionContentRenderer"]["items"][2]["videoDescriptionMusicSectionRenderer"]["carouselLockups"].as_array() {
          Some(carousel_lockups) => {
            let mut songs = Vec::<MusicTrack>::new();
            for lockup in carousel_lockups {
              let song_title = match lockup["carouselLockupRenderer"]["videoLockup"]["compactVideoRenderer"]["title"]["runs"][0]["text"].as_str() {
                Some(title) => Some(String::from(title)),
                None => None
              };
              let video_id = match lockup["carouselLockupRenderer"]["videoLockup"]["compactVideoRenderer"]["navigationEndpoint"]["watchEndpoint"]["videoId"].as_str() {
                Some(video_id) => Some(String::from(video_id)),
                None => None
              };
              let (artist, album, licenses) = match lockup["carouselLockupRenderer"]["infoRows"].as_array() {
                Some(rows) => {
                  let mut artist = None::<String>;
                  let mut album = None::<String>;
                  let mut licences = None::<String>;
                  for row in rows {
                    let status_key = match row["infoRowRenderer"]["infoRowExpandStatusKey"].as_str() {
                      Some(status_key) => Some(status_key),
                      None => None
                    };
                    match status_key {
                      Some(status_key) => {
                        if status_key == "structured-description-music-section-artists-row-state-id" {
                          match row["infoRowRenderer"]["defaultMetadata"]["simpleText"].as_str() {
                            Some(artist_name) => {
                              artist = Some(String::from(artist_name));
                            },
                            None => {}
                          }
                        } else if status_key == "structured-description-music-section-licenses-row-state-id" {
                          match row["infoRowRenderer"]["expandedMetadata"]["simpleText"].as_str() {
                            Some(license) => {
                              licences = Some(String::from(license));
                            },
                            None => {}
                          }
                        }
                      },
                      None => {
                        // album info
                        match row["infoRowRenderer"]["defaultMetadata"]["simpleText"].as_str() {
                          Some(album_name) => {
                            album = Some(String::from(album_name));
                          },
                          None => {}
                        };
                      }
                    }
                  }
                  (artist, album, licences)
                },
                None => (None, None, None)
              };
              songs.push(MusicTrack {
                song: song_title,
                artist: artist,
                album: album,
                license: licenses,
                video_id: video_id
              })
            }
            Some(songs)
          },
          None => None
        };
        match result {
          Some(result) => return Some(result),
          _ => {}
        };
      }
      None
    },
    None => None
  }
}

#[derive(Deserialize, Serialize)]
pub struct CommentContinuation {
  pub title: String,
  pub token: String
}

pub fn get_comment_continuations(json: &Value) -> Option<Vec::<CommentContinuation>> {
  match json["engagementPanels"].as_array() {
    Some(panels) => {
      for i in 0..panels.len() {
        match panels[i]["engagementPanelSectionListRenderer"]["header"]["engagementPanelTitleHeaderRenderer"]["menu"]["sortFilterSubMenuRenderer"]["subMenuItems"].as_array() {
          Some(sub_menu_items) => {
            return Some(sub_menu_items.into_iter().filter_map(|item| {
              let Some(title) = item["title"].as_str() else { return None };
              let Some(token) = item["serviceEndpoint"]["continuationCommand"]["token"].as_str() else { return None };

              Some(CommentContinuation {
                title: String::from(title),
                token: String::from(token)
              })
            }).collect::<Vec::<CommentContinuation>>());
          },
          None => {}
        };
      }
    },
    None => {}
  };
  None
}
