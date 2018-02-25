extern crate youtube_api;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env::args;

fn main() {
    //youtube_api::requests::get("http://hw2.aberinger.me", vec![("rc", "")]).unwrap();
    
    let file = File::open(".key").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut key = String::new();
    let _ = buf_reader.read_to_string(&mut key);

    let api = youtube_api::YoutubeAPI::new(key.trim(), true);
    let args: Vec<String> = args().collect();

    let query: &str;

    if args.len() > 1 {
        query = &args[1];
    } else {
        eprintln!("Usage: {} query", &args[0]);
        std::process::exit(1);
    }

    println!("{}", api.get_video_url(query));
}
