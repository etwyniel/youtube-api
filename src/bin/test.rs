extern crate youtube_api;

fn main() {
    youtube_api::requests::get("http://hw2.aberinger.me", vec![("rc", "")]).unwrap();
}
