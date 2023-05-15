
use regex::Regex;
use std::str::FromStr;
use urlencoding::decode;
use std::num::ParseIntError;
use std::fmt::{Formatter, Display};
#[cfg(feature = "decipher_streams")]
use boa_engine::{Context};

pub fn extract_sig_timestamp(player_res: &str) -> Result<i32, ParseIntError> {
  let re = Regex::new(r"signatureTimestamp:([^,]*),").unwrap();
  match re.captures(player_res) {
    Some(captures) => {
      let timestamp = match captures.get(1) { Some(group) => group.as_str(), None => "" };
      i32::from_str(timestamp)
    },
    None => {// might be a better way to do this
      i32::from_str("")// will be an error
    }
  }
}

pub enum ExtractSigJsCodeError {
  NoObjectNameFound,
  NoFunctionsOrCallsFound
}

impl Display for ExtractSigJsCodeError {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    write!(f, "{}", match self {
      ExtractSigJsCodeError::NoObjectNameFound => "No object name found when extracting sig js code",
      ExtractSigJsCodeError::NoFunctionsOrCallsFound => "No functions or calls found when extracting sig js code"
    })
  }
}

pub fn extract_sig_js_code(player_res: &str) -> Result<String, ExtractSigJsCodeError> {
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
    (None, _, _) => {
      Err(ExtractSigJsCodeError::NoObjectNameFound)
    },
    _ => {
      Err(ExtractSigJsCodeError::NoFunctionsOrCallsFound)
    }
  }
  
} 

pub enum ExtractNsigJsCodeError {
  NoNsigFound
}

impl Display for ExtractNsigJsCodeError {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    write!(f, "{}", match self {
      ExtractNsigJsCodeError::NoNsigFound => "No nsig code found",
    })
  }
}

pub fn extract_nsig_js_code(player_res: &str) -> Result<String, ExtractNsigJsCodeError> {
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
      Err(ExtractNsigJsCodeError::NoNsigFound)
    }
  }
}

pub enum CreateFormattableDecipherJsError {
  ErrorExtractingSig(ExtractSigJsCodeError),
  ErrorExtractingNsig(ExtractNsigJsCodeError)
}

impl Display for CreateFormattableDecipherJsError {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    write!(f, "{}", match self {
      CreateFormattableDecipherJsError::ErrorExtractingSig(e) => format!("{}", e),
      CreateFormattableDecipherJsError::ErrorExtractingNsig(e) => format!("{}", e),
    })
  }
}

pub fn create_formatable_decipher_js_code(player_res: &str) -> Result<String, CreateFormattableDecipherJsError> {
  let mut errors = Vec::<String>::new();
  let sig_js = match extract_sig_js_code(player_res) {
    Ok(result) => result,
    Err(e) => {
      return Err(CreateFormattableDecipherJsError::ErrorExtractingSig(e)) 
    }
  };
  let nsig_js = match extract_nsig_js_code(player_res) {
    Ok(result) => result,
    Err(e) => {
      return Err(CreateFormattableDecipherJsError::ErrorExtractingNsig(e)) 
    }
  };
  Ok(format!(r#"{}; {};
var deciphered_sig = decipher_sig(s);
var deciphered_nsig = decipher_nsig(n);
var deciphered_url = `${{url.replace(`&n=${{n}}`, `&n=${{deciphered_nsig}}`)}}&${{sp}}=${{encodeURIComponent(deciphered_sig)}}`;
deciphered_url"#, sig_js, nsig_js))
}

pub enum CreateExecutableDecipherJsError {
  ErrorCreatingFormattableCode(CreateFormattableDecipherJsError),
  ErrorFormatingCodeIntoExecutable(FormatDecipherCodeError)
}

impl Display for CreateExecutableDecipherJsError {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    write!(f, "{}", match self {
      CreateExecutableDecipherJsError::ErrorCreatingFormattableCode(e) => format!("{}", e),
      CreateExecutableDecipherJsError::ErrorFormatingCodeIntoExecutable(e) => format!("{}", e),
    })
  }
}

pub fn create_executable_decipher_js_code(ciphered_url: &str, player_res: &str) -> Result<String, CreateExecutableDecipherJsError> {
  match create_formatable_decipher_js_code(player_res) {
    Ok(formatable_js_code) => {
      match format_decipher_code_into_executable(ciphered_url, &formatable_js_code) {
        Ok(decipher_code) => Ok(decipher_code),
        Err(error) => Err(CreateExecutableDecipherJsError::ErrorFormatingCodeIntoExecutable(error))
      }
    },
    Err(error) => {
      Err(CreateExecutableDecipherJsError::ErrorCreatingFormattableCode(error))
    }
  }
}

pub fn get_s_sp_n_url(signature_cipher: &str) -> (Option<String>, Option<String>, Option<String>, Option<String>) {
  let s_re = Regex::new(r"s=(.*?)&").unwrap();
  let s = match s_re.captures(signature_cipher) {
    Some(captures) => {
      match decode(captures.get(1).unwrap().as_str()) {
        Ok(decoded_s) => Some(format!("{}", decoded_s)),
        Err(_) => None
      }
    },
    None => {
      None
    }
  };
  let sp_re = Regex::new(r"&sp=(.*?)&").unwrap();
  let sp = match sp_re.captures(signature_cipher) {
    Some(captures) => {
      Some(String::from(captures.get(1).unwrap().as_str()))
    },
    None => {
      None
    }
  };
  let url_re = Regex::new(r"&url=(.*)").unwrap();
  let encoded_url = match url_re.captures(signature_cipher) {
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
        Some(captures) => Some(String::from(captures.get(1).unwrap().as_str())),
        None => None
      }
    },
    None => None
  };
  (s,sp,n,decoded_url)
}

pub enum FormatDecipherCodeError {
  ErrorParsingSignature
}

impl Display for FormatDecipherCodeError {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    write!(f, "{}", match self {
      FormatDecipherCodeError::ErrorParsingSignature => "Error parsing signatureCipher",
    })
  }
}

pub fn format_decipher_code_into_executable(ciphered_url: &str, js_code: &str) -> Result<String, FormatDecipherCodeError> {
  let (s,sp,n,decoded_url) = get_s_sp_n_url(ciphered_url);
  match (decoded_url.clone(), s, sp, n) {
    (Some(decoded_url), Some(s), Some(sp), Some(n)) => {
      Ok(format!(r#"var url = "{}";
      var s = "{}";
      var sp = "{}";
      var n = "{}";
      {}"#, decoded_url, s, sp, n, js_code))
    },
    _ => {
      Err(FormatDecipherCodeError::ErrorParsingSignature)
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
    Err(error) => Err(format!("{}", error))
  }
}

#[cfg(feature = "decipher_streams")]
pub fn decipher_stream(ciphered_url: &str, player_res: &str) -> Result<String, String> {
  match create_executable_decipher_js_code(ciphered_url, player_res) {
    Ok(mut js_code) => {
      run_js_in_boa(js_code)
    },
    Err(error) => {
      Err(format!("{}", error))
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
