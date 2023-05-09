# Examples

### Fetching the next endpoint for video information
```rust
use yayti::extractors::innertube::fetch_next;
use yayti::parsers::{ClientContext, web::video::{get_title, get_description, get_description_html}};
use serde_json::{from_str, Value};

// ClientContext is the context used to interface with innertube that contains relevant information such as the client version or user agent
let Ok(next) = fetch_next("QrGrOK8oZG8", &ClientContext::default_web(), Some("en")).await else { todo!() };
let Ok(next_value) = from_str::<Value>(&next) else { todo!() };
let Some(title) = get_title(&next_value) else { todo!() };
let Some(description) = get_description(&next_value) else { todo!() };
let Some(description_html) = get_description_html(&next_value) else { todo!() };

println!("title: {}", title);
// Too Many Cooks | Adult Swim
println!("description: {}", description);
// Too Many Cooks
// Watch Full Episodes: http://asw.im/3cyX3a . . .
println!("descriptionHtml:");
println!("{}", description_html);
// Too Many Cooks
// Watch Full Episodes: <a href="http://asw.im/3cyX3a"> . . .
```
### Fetching and deciphering streams
requires feature: `decipher_streams`
```rust
use yayti::extractors::{ciphers::get_player_js_id, ciphers::get_player_response, innertube::fetch_player_with_sig_timestamp};
use yayti::parsers::{ClientContext, ciphers::{extract_sig_timestamp, decipher_stream}, web::video::{get_legacy_formats}};
use serde_json::{from_str, Value};

// this is used to fetch the code to decipher streams
let Ok(player_js_id) = get_player_js_id().await else { todo!() };
// you should request it every time

// this is the code to decipher streams
let Ok(player_js_response) = get_player_response(&player_js_id).await else { todo!() };
// you should only request it again when the player_js_id changes

// this is needed to fetch the player endpoint
let signature_timestamp = extract_sig_timestamp(&player_js_response);

let Ok(player) = fetch_player_with_sig_timestamp("wuJIqmha2Hk", signature_timestamp, &ClientContext::default_web(), Some("en")).await else { todo!() };
let Ok(player_value) = from_str::<Value>(&player) else { todo!() };

// this also works with `get_adaptive_formats`
let Some(legacy_formats) = get_legacy_formats(&player_value) else { todo!() };
let Some(signature_cipher) = &legacy_formats[0].signature_cipher else { todo!() };

let Ok(deciphered_url) = decipher_stream(signature_cipher, &player_js_response) else { todo!() };
println!("{}", deciphered_url);
```
### Generating stream deciphering code for execution by your preferred JS runtime
```rust
use yayti::extractors::{ciphers::get_player_js_id, ciphers::get_player_response, innertube::fetch_player_with_sig_timestamp};
use yayti::parsers::{ClientContext, ciphers::{extract_sig_timestamp, create_executable_decipher_js_code}, web::video::{get_legacy_formats}};
use serde_json::{from_str, Value};

// this is used to fetch the code to decipher streams
let Ok(player_js_id) = get_player_js_id().await else { todo!() };
// you should request it every time

// this is the code to decipher streams
let Ok(player_js_response) = get_player_response(&player_js_id).await else { todo!() };
// you should only request it again when the player_js_id changes

// this is needed to fetch the player endpoint
let signature_timestamp = extract_sig_timestamp(&player_js_response);

let Ok(player) = fetch_player_with_sig_timestamp("wuJIqmha2Hk", signature_timestamp, &ClientContext::default_web(), Some("en")).await else { todo!() };
let Ok(player_value) = from_str::<Value>(&player) else { todo!() };

// this also works with `get_adaptive_formats`
let Some(legacy_formats) = get_legacy_formats(&player_value) else { todo!() };
let Some(signature_cipher) = &legacy_formats[0].signature_cipher else { todo!() };
let Ok(js_code) = create_executable_decipher_js_code(&signature_cipher, &player_js_response) else { todo!() };

// and there you have it 👐
// ready to be copy and pasted into a browser's inspect element
// or whatever is your preferred means of JS execution
println!("{}", js_code);
```
