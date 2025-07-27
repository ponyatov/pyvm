#![allow(dead_code)]
#![allow(unused_variables)]

mod config;

use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

// HTTP return codes
const HTTP_200_OK: &[u8] = b"HTTP/1.1 200 OK\r\n";
const HTTP_404_NOTFOUND: &[u8] = b"HTTP/1.1 404 Not Found\r\n";

// MIME types
const TEXT_HTML: &[u8] = b"Content-Type: text/html\r\n";
const TEXT_PLAIN: &[u8] = b"Content-Type: text/plain\r\n";
const TEXT_CSS: &[u8] = b"Content-Type: text/css\r\n";
const TEXT_JS: &[u8] = b"Content-Type: application/javascript\r\n";
const IMAGE_PNG: &[u8] = b"Content-Type: image/png\r\n";

fn error_404(client: &mut TcpStream, method: &[u8], url: &[u8]) {
    client.write(&HTTP_404_NOTFOUND).unwrap();
    client.write(&TEXT_HTML).unwrap();
    client.write(&INDEX_HEAD).unwrap();
    client.write(b"\r\nmethod: ").unwrap();
    client.write(method).unwrap();
    client.write(b" ").unwrap();
    client.write(url).unwrap();
    client.flush().unwrap();
}

// static content
const INDEX_HEAD: &[u8] = include_bytes!("../static/head.html");
const INDEX_BODY: &[u8] = include_bytes!("../static/body.html");
const INDEX_FOOT: &[u8] = include_bytes!("../static/foot.html");
const LOGO_PNG: &[u8] = include_bytes!("../doc/logo.png");
const CSS_CSS: &[u8] = include_bytes!("../static/css.css");
const JS_JS: &[u8] = include_bytes!("../static/js.js");
const JQUERY_MIN_JS: &[u8] = include_bytes!("../static/cdn/jquery.min.js");

fn logo(client: &mut TcpStream) {
    client.write(&HTTP_200_OK).unwrap();
    client.write(&IMAGE_PNG).unwrap();
    client.write(b"\r\n").unwrap();
    client.write(&LOGO_PNG).unwrap();
    client.flush().unwrap();
}

fn router(client: &mut TcpStream) {
    let mut buffer = [0; 1024];
    client.read(&mut buffer).unwrap();

    let request = buffer.split(|&x| x == b'\n').next().unwrap();
    let parts: Vec<&[u8]> = request.split(|&x| x == b' ').collect();
    let (method, url) = (parts[0], parts[1]);

    match (method, url) {
        // (b"GET", b"/") | (b"GET", b"/index.html") => index(client),
        (b"GET", b"/favicon.ico") | (b"GET", b"/logo.png") => logo(client),
        // (b"GET", b"/css.css") => css(client),
        // (b"GET", b"/js.js") => js(client),
        // (b"GET", b"/jquery.min.js") => jquery(client),
        _ => error_404(client, method, url),
    }
}

pub fn main() {
    let listener = TcpListener::bind(config::BIND).unwrap();
    eprintln!("server @ http://{}:{}", config::IP, config::PORT);
    for client in listener.incoming() {
        match client {
            Ok(mut client) => {
                thread::spawn(move || router(&mut client));
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
