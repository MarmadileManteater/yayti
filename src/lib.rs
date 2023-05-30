
mod constants;
pub mod helpers;
pub mod extractors;
pub mod parsers;



#[cfg(test)]
mod tests {

  use serial_test::serial;
  use extractors::innertube::*;
  use extractors::scrape::video::*;
  use parsers::*;

  use super::*;
  
  #[tokio::test]
  #[serial]
  async fn scrape_video_parse() {
    let res = scrape_video_page("JgADogL6Bp8", Some("cs")).await.unwrap();
    let page_data = serde_json::from_str::<serde_json::Value>(&res.next).unwrap();
    let video_title = web::video::get_title(&page_data).unwrap();
    let description = web::video::get_description(&page_data).unwrap();
    let description_html = web::video::get_description_html(&page_data).unwrap();
    #[cfg(feature = "parse_languages_to_published")]
    let published = web::video::get_published(&page_data, "cs").unwrap();
    assert_eq!("This is Pure False Advertising - HP 550 Keyboard", video_title);
    assert_eq!(true, description.contains("It’s no secret their chairs are great!"));
    assert_eq!(true, description_html.contains(r#"OUR WAN PODCAST GEAR: <a href="https://lmg.gg/wanset">"#));
    assert_eq!(true, description_html.contains(r#"Artist Link: <a href="http://www.youtube.com/approachingnirvana">"#));
    assert_eq!(true, description_html.contains(r#"<a href="https://youtu.be/PKfxmFU3lWY">"#));
    #[cfg(feature = "parse_languages_to_published")]
    assert_eq!(1681603200, published);
  }


  #[tokio::test]
  #[serial]
  async fn innertube_fetch_next_video_parse_web() {
    let page_data = serde_json::from_str::<serde_json::Value>(&fetch_next("JgADogL6Bp8", &ClientContext::default_web(), None).await.unwrap()).unwrap();
    let video_title = web::video::get_title(&page_data).unwrap();
    let description = web::video::get_description(&page_data).unwrap();
    let description_html = web::video::get_description_html(&page_data).unwrap();
    #[cfg(feature = "parse_languages_to_published")]
    let published = web::video::get_published(&page_data, "en").unwrap();
    assert_eq!("This is Pure False Advertising - HP 550 Keyboard", video_title);
    assert_eq!(true, description.contains("It’s no secret their chairs are great!"));
    assert_eq!(true, description_html.contains(r#"OUR WAN PODCAST GEAR: <a href="https://lmg.gg/wanset">"#));
    assert_eq!(true, description_html.contains(r#"Artist Link: <a href="http://www.youtube.com/approachingnirvana">"#));
    assert_eq!(true, description_html.contains(r#"<a href="https://youtu.be/PKfxmFU3lWY">"#));
    #[cfg(feature = "parse_languages_to_published")]
    assert_eq!(1681603200, published);
  }

  #[tokio::test]
  #[serial]
  async fn innertube_fetch_next2_video_parse_web() {
    let page_data = serde_json::from_str::<serde_json::Value>(&fetch_next("S4XMBz_mfkQ", &ClientContext::default_web(), Some("cs")).await.unwrap()).unwrap();
    let video_title = web::video::get_title(&page_data).unwrap();
    let description = web::video::get_description(&page_data).unwrap();
    let description_html = web::video::get_description_html(&page_data).unwrap();
    #[cfg(feature = "parse_languages_to_published")]
    let published = web::video::get_published(&page_data, "cs").unwrap();
    assert_eq!("Fast ASMR Does the BEST Triggers with Me! | In-Person ASMR Session", video_title);
    assert_eq!(true, description.contains("Start speaking a new language in 3 weeks with Babbel 🎉"));
    assert_eq!(true, description_html.contains(r#"➡️  HERE: <a href="https://go.babbel.com/t?bsc=1200m60-youtube-gibiasmr"#));
    #[cfg(feature = "parse_languages_to_published")]
    assert_eq!(1681516800, published);
  }

  #[tokio::test]
  #[serial]
  async fn innertube_fetch_next_video_parse_web_priemere_published() {
    let page_data = serde_json::from_str::<serde_json::Value>(&fetch_next("VLn5ehm2kD4", &ClientContext::default_web(), Some("eu")).await.unwrap()).unwrap();
    let video_title = web::video::get_title(&page_data).unwrap();
    let description = web::video::get_description(&page_data).unwrap();
    let description_html = web::video::get_description_html(&page_data).unwrap();
    #[cfg(feature = "parse_languages_to_published")]
    let published = web::video::get_published(&page_data, "eu").unwrap();
    assert_eq!(r#"🌸 ASMR: "Let's Stim Together!!" Under-Stimulation Comfort"#, video_title);
    assert_eq!(true, description.contains("◑﹏◐"));
    assert_eq!(true, description_html.contains(r#"~ <a href="https://linktr.ee/8bitauthor"#));
    #[cfg(feature = "parse_languages_to_published")]
    assert_eq!(1659744000, published);
  }

  #[tokio::test]
  #[serial]
  async fn innertube_fetch_next_video_parse_android() {
    let page_data = serde_json::from_str::<serde_json::Value>(&fetch_next("cd4clAWxk6Q", &ClientContext::default_android(), None).await.unwrap()).unwrap();
    let video_title = android::video::get_title(&page_data).unwrap();
    assert_eq!("Retrofuturism and Video Games: The Future in Gaming 1950-1974", video_title);
  }

  #[tokio::test]
  #[serial]
  async fn innertube_fetch_player_video_parse_android() {
    let page_data = serde_json::from_str::<serde_json::Value>(&fetch_player("G68Q4lCM5pQ", &ClientContext::default_android(), None).await.unwrap()).unwrap();
    let video_title = android::video::get_title(&page_data).unwrap();
    let description = android::video::get_description(&page_data).unwrap();
    assert_eq!("japanese jazz when driving on a warm night", video_title);
    assert_eq!(true, description.contains("ow! mosquitos :("));
  }
 
  #[tokio::test]
  #[serial]
  async fn innertube_fetch_next_video_parse_tv() {
    let page_data = serde_json::from_str::<serde_json::Value>(&fetch_next("Mwt35SEeR9w", &ClientContext::default_tv(), None).await.unwrap()).unwrap();
    let video_title = tv::video::get_title(&page_data).unwrap();
    let description = tv::video::get_description(&page_data).unwrap();
    let description_html = tv::video::get_description_html(&page_data).unwrap();
    assert_eq!("What pretending to be crazy looks like", video_title);
    assert_eq!(true, description.contains("Does the demon have an attorney?"));
    assert_eq!(true, description_html.contains(r#"<a href="https://www.youtube.com/c/TRUECRIMELoser">https://www.youtube.com/c/TRUECRIMELoser</a>"#));
    assert_eq!(true, description_html.contains(r#"<a href="https://www.youtube.com/user/kizzume">https://www.youtube.com/user/kizzume</a>"#));
  }

}
