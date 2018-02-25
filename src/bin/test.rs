extern crate youtube_api;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    //youtube_api::requests::get("http://hw2.aberinger.me", vec![("rc", "")]).unwrap();
    
    let mut file = File::open(".key").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut key = String::new();
    buf_reader.read_to_string(&mut key);

    let api = youtube_api::YoutubeAPI{key: key.trim().to_string(), single_request: true};

    println!("{}", api.get_video_url("hiphockers"));
}
