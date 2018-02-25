pub mod requests;

pub struct YoutubeAPI {
    key: String,
    single_request: bool
}

impl YoutubeAPI {
    pub fn new(key: &str, single_request: bool) -> YoutubeAPI {
        YoutubeAPI {key: key.to_string(), single_request}
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
