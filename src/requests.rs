extern crate openssl;

use std::net::TcpStream;
use std::io::prelude::*;
use self::openssl::ssl::{SslMethod, SslConnector};

pub fn url_encode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    for c in bytes {
        out += &match c {
            an @ &b'A' ... b'Z' | an @ &b'a' ... b'z' | an @ &b'0' ... b'9' =>
                (*an as char).to_string(),
            other => format!("%{:02X}", other)
        };
    }
    /*
    for c in bytes {
        if (c >= b'A' && c <= b'Z') ||
           (c >= b'a' && c <= b'z') ||
           (c >= b'0' && c <= b'9')
    }
    */
    out
}

fn get_hostname_and_path(url: &str) -> Result<(String, String), ()> {
    let start = match url.find(':') {
        Some(i) => i + 3,
        None => {return Err(());}
    };
    let end = match url[start..].find('/') {
        Some(i) => i + start,
        None => url.len()
    };
    let path = (if end == url.len() {"/"} else {&url[end..url.len()]}).to_string();
    Ok((url[start..end].to_string(), path))
}

fn build_path(path: &str, mut args: Vec<(&str, &str)>) -> String {
    let mut path = path.to_string();
    if args.len() == 0 {return path;}
    path.push('?');
    let (k1, v1) = args.pop().unwrap();
    path += &url_encode(k1);
    if v1.len() > 0 {
        path.push('=');
        path += &url_encode(v1);
    }
    for (key, val) in args {
        path.push('&');
        path += &key.replace(' ', "+");
        if val.len() > 0 {
            path.push('=');
            path += &val.replace(' ', "+");
        }
    }
    path
}

pub trait Stream: Read + Write {}
impl<T> Stream for T where T: Read + Write {}

fn get_stream(host: &str, url: &str) -> Result<Box<Stream>, ()> {
    if url.starts_with("https") {
        eprintln!("Using https.");
        let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
        let base = TcpStream::connect(&format!("{}:443", host)).unwrap();
        Ok(Box::new(connector.connect(host, base).unwrap()))
    } else {
        Ok(Box::new(TcpStream::connect(&format!("{}:80", host)).unwrap()))
    }
}

fn get_raw(url: &str, args: Vec<(&str, &str)>) -> Result<String, ()> {
    let (host, path) = get_hostname_and_path(&url)?;
    let path = build_path(&path, args);

    let request = format!("GET {1} HTTP/1.1{0}Host: {2}{0}User-Agent: \
        rust-get/0.1.0{0}Accept: */*{0}Content-Length: 0{0}\
        Connection: close{0}{0}",
        "\r\n", path, host);

    //println!("Headers:\n{}\n", request);

    let mut stream = get_stream(&host, url).unwrap();
    let _ = stream.write_all(request.as_bytes());
    let _ = stream.flush();

    let mut out = String::new();
    let _ = stream.read_to_string(&mut out);

    return Ok(out);
}

pub struct Response {
    pub code: i32,
    pub status: String,
    pub text: String
}

pub fn get(url: &str, args: Vec<(&str, &str)>) -> Result<Response, ()> {
    let resp = get_raw(url, args)?;
    let mut header_and_body = resp.split("\r\n\r\n");
    let (header, body): (&str, &str) = (header_and_body.next().unwrap(),
                                        header_and_body.next().unwrap());
    let lines: Vec<&str> = header.splitn(2, "\r\n").collect();
    let mut first_line = lines[0].splitn(3, ' ');
    let (_http_ver, code, status): (&str, &str, &str) = (first_line.next().unwrap(),
                                                        first_line.next().unwrap(),
                                                        first_line.next().unwrap());
    let code = match code.parse::<i32>() {
        Ok(i) => i,
        Err(_) => { return Err(()); }
    
    };
    //println!("Version: {}\nCode: {}\nStatus: {}\n", http_ver, code, status);
    //println!("Body:\n{}", body);
    
    Ok(Response{code: code, status: status.to_string(), text: body.to_string()})
}

