
use regex::Regex;
use std::str::FromStr;
use urlencoding::decode;
#[cfg(feature = "decipher_streams")]
use boa_engine::{Context};

pub fn extract_sig_timestamp(player_res: &str) -> i32 {
  let re = Regex::new(r"signatureTimestamp:([^,]*),").unwrap();
  match re.captures(player_res) {
    Some(captures) => {
      let timestamp = captures.get(1).unwrap().as_str();
      match i32::from_str(timestamp) {
        Ok(timestamp_number) => timestamp_number,
        Err(_) => 0
      }
    },
    None => {
      0
    }
  }
}

pub fn extract_sig_js_code(player_res: &str) -> Result<String, String> {
  let calls_regex = Regex::new(r#"function\(a\)\{a=a.split\(""\)(.*?)return a.join\(""\)\}"#).unwrap();
  let calls = match calls_regex.captures(player_res) {
    Some(captures) => {
      Some(captures.get(1).unwrap().as_str())
    },
    None => {
      None
    }
  };
  let object_name_regex = Regex::new(r"(.*?)\.|\[").unwrap();
  let object_name = match object_name_regex.captures(calls.unwrap_or("")) {
    Some(captures) => {
      Some(String::from(captures.get(1).unwrap().as_str().replace(";", "").trim()))
    },
    None => {
      None
    }
  };
  let functions_regex = Regex::new(&format!(r"(?m)var {}=\{{([\s\S]*?)\}};", object_name.clone().unwrap_or(String::from("")))).unwrap();
  let functions = match functions_regex.captures(player_res) {
    Some(captures) => {
      Some(captures.get(1).unwrap().as_str())
    },
    None => {
      None
    }
  };
  match (object_name, functions, calls) {
    (Some(object_name), Some(functions), Some(calls)) => {
      Ok(format!(r#"function decipher_sig(a) {{ a = a.split(""); let {}={{{}}}{} return a.join("") }}"#, object_name, functions, calls))
    },
    (Some(object_name), None, Some(calls)) => {
      Ok(format!(r#"function decipher_sig(a) {{ a = a.split(""); let {}={{}}{} return a.join("") }}"#, object_name, calls))
    },
    (Some(object_name), Some(functions), None) => {
      Ok(format!(r#"function decipher_sig(a) {{ a = a.split(""); let {}={{{}}} return a.join("") }}"#, object_name, functions))
    },
    _ => {
      Err(String::from("ERROR: object name, or functions and calls missing"))
    }
  }
  
} 

pub fn extract_nsig_js_code(player_res: &str) -> Result<String, String> {
  let nsig_regex = Regex::new(r#"b=a.split\(""\)([\s\S]*?)}return b\.join\(""\)"#).unwrap();
  let nsig = match nsig_regex.captures(player_res) {
    Some(captures) => {
      Some(String::from(captures.get(1).unwrap().as_str()))
    },
    None => {
      None
    }
  };
  match nsig {
    Some(nsig) => {
      Ok(format!(r#"function decipher_nsig(a) {{ let b=a.split(""){}}} return b.join("")}}"#, nsig))
    },
    None => {
      Err(String::from("nsig failed to extract from player.js"))
    }
  }
}

pub fn create_formatable_decipher_js_code(player_res: &str) -> Result<String, String> {
  let mut errors = Vec::<String>::new();
  let sig_js = match extract_sig_js_code(player_res) {
    Ok(sig_js) => {
      Some(sig_js)
    },
    Err(error) => {
      errors.push(error);
      None
    }
  };
  let nsig_js = match extract_nsig_js_code(player_res) {
    Ok(nsig_js) => {
      Some(nsig_js)
    },
    Err(error) => {
      errors.push(error);
      None
    }
  };
  if errors.len() == 0 {
    Ok(format!(r#"{}; {};
var deciphered_sig = decipher_sig(s);
var deciphered_nsig = decipher_nsig(n);
var deciphered_url = `${{url.replace(`&n=${{n}}`, `&n=${{deciphered_nsig}}`)}}&${{sp}}=${{encodeURIComponent(deciphered_sig)}}`;
deciphered_url"#, sig_js.unwrap(), nsig_js.unwrap()))
  } else {
    Err(errors.join("\r\n"))
  }
}

pub fn create_executable_decipher_js_code(ciphered_url: &str, player_res: &str) -> Result<String, String> {
  match create_formatable_decipher_js_code(player_res) {
    Ok(formatable_js_code) => {
      format_decipher_code_into_executable(ciphered_url, &formatable_js_code)
    },
    Err(error) => {
      Err(error)
    }
  }
}

pub fn format_decipher_code_into_executable(ciphered_url: &str, js_code: &str) -> Result<String, String> {
  let s_re = Regex::new(r"s=(.*?)&").unwrap();
  let s = match s_re.captures(ciphered_url) {
    Some(captures) => {
      Some(decode(captures.get(1).unwrap().as_str()).unwrap())
    },
    None => {
      None
    }
  };
  let sp_re = Regex::new(r"&sp=(.*?)&").unwrap();
  let sp = match sp_re.captures(ciphered_url) {
    Some(captures) => {
      Some(captures.get(1).unwrap().as_str())
    },
    None => {
      None
    }
  };
  let url_re = Regex::new(r"&url=(.*)").unwrap();
  let encoded_url = match url_re.captures(ciphered_url) {
    Some(captures) => {
      Some(captures.get(1).unwrap().as_str())
    },
    None => {
      None
    }
  };
  let decoded_url = match encoded_url {
    Some(encoded_url) => Some(String::from(decode(encoded_url).unwrap())),
    None => None
  };
  let n_re = Regex::new(r"&n=(.*?)&").unwrap();
  let n = match decoded_url {
    Some(ref decoded_url) => {
      match n_re.captures(decoded_url) {
        Some(captures) => Some(captures.get(1).unwrap().as_str()),
        None => None
      }
    },
    None => None
  };
  match (decoded_url.clone(), s, sp, n) {
    (Some(decoded_url), Some(s), Some(sp), Some(n)) => {
      Ok(format!(r#"var url = "{}";
      var s = "{}";
      var sp = "{}";
      var n = "{}";
      {}"#, decoded_url, s, sp, n, js_code))
    },
    _ => {
      Err(String::from("Error parsing ciphered url"))
    }
  }

}

#[cfg(feature = "decipher_streams")]
pub fn decipher_streams(ciphered_urls: Vec::<String>, player_res: &str) -> Result<Vec::<Option<String>>, String> {
  match create_formatable_decipher_js_code(player_res) {
    Ok(formatable_js_code) => {
      Ok(ciphered_urls.into_iter().map(|ciphered_url| {
        match format_decipher_code_into_executable(&ciphered_url, &formatable_js_code) {
          Ok(executable_js_code) => {
            match run_js_in_boa(executable_js_code) {
              Ok(result) => {
                Some(result)
              },
              Err(_) => None
            }
          },
          Err(_) => None
        }
      }).collect::<Vec::<Option<String>>>())
    },
    Err(error) => Err(error)
  }
}

#[cfg(feature = "decipher_streams")]
pub fn decipher_stream(ciphered_url: &str, player_res: &str) -> Result<String, String> {
  match create_executable_decipher_js_code(ciphered_url, player_res) {
    Ok(mut js_code) => {
      run_js_in_boa(js_code)
    },
    Err(error) => {
      Err(error)
    }
  }
}

#[cfg(feature = "decipher_streams")]
pub fn run_js_in_boa(mut js_code: String) -> Result<String, String> {
  // surround regex with quotes because 🐍boa likes it that way
  let needs_quotes_re = Regex::new(r"\/\\+.*\/").unwrap();
  match needs_quotes_re.captures(&String::from(&js_code)) {
    Some(matches) => {
      for matche in matches.iter() {
        let str_match = matche.unwrap().as_str();
        js_code = js_code.replace(str_match, &format!("`{}`", &str_match));
      }
    },
    None => {}
  };
  // 🚗 run the decipher JS in boa
  let mut context = Context::default();
  match context.eval(&js_code) {
    Ok(result) => {
      let str_result = result.to_string(&mut context);
      match str_result {
        Ok(result) => {
          Ok(format!("{}", result.as_str()))
        },
        Err(_) => {
          Err(String::from("error parsing JS output"))
        }
      }
    },
    Err(_) => {
      Err(String::from("error running JS"))
    }
  }
} 
