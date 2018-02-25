extern crate serde_json;
use serde_json::{Value, Error};

pub mod requests;

pub struct YoutubeAPI {
    pub key: String,
    pub single_request: bool
}

impl YoutubeAPI {
    pub fn new(key: &str, single_request: bool) -> YoutubeAPI {
        YoutubeAPI {key: key.to_string(), single_request}
    }

    pub fn get_video_url(&self, query: &str) -> String {
        let mut args = vec![("key", &self.key[..]),
                        ("type", "video"),
                        ("q", query),
                        ("part", "snippet")];
        let resp = requests::get("https://www.googleapis.com/youtube/v3/search", args).unwrap();
        let json: Value = serde_json::from_str(&resp.text[..]).unwrap();
        let id = &json["items"][0]["id"]["videoId"];

        format!("https://youtube.com/watch?v={}", id.as_str().unwrap()).to_string()
    }
}

#[test]
fn get_hostname_test() {
    let (hostname, path) = get_hostname_and_path("https://reddit.com/r/askreddit").unwrap();
    assert_eq!(&hostname, "reddit.com");
    assert_eq!(&path, "/r/askreddit");

    let (hostname, path) = get_hostname_and_path("http://reddit.com").unwrap();
    assert_eq!(&hostname, "reddit.com");
    assert_eq!(&path, "/");
}
