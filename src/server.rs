#![allow(dead_code)]
#![allow(unused_variables)]

mod config;

use std::net::TcpListener;

// HTTP return codes
const HTTP_200_OK: &[u8] = b"HTTP/1.1 200 OK\r\n";
const HTTP_404_NOTFOUND: &[u8] = b"HTTP/1.1 404 Not Found\r\n";

// MIME types
const TEXT_HTML: &[u8] = b"Content-Type: text/html\r\n";
const TEXT_PLAIN: &[u8] = b"Content-Type: text/plain\r\n";
const TEXT_CSS: &[u8] = b"Content-Type: text/css\r\n";
const TEXT_JS: &[u8] = b"Content-Type: application/javascript\r\n";
const IMAGE_PNG: &[u8] = b"Content-Type: image/png\r\n";

// static content
const INDEX_HTML: &[u8] = include_bytes!("../static/index.html");
const LOGO_PNG: &[u8] = include_bytes!("../doc/logo.png");
const CSS_CSS: &[u8] = include_bytes!("../static/css.css");
const JS_JS: &[u8] = include_bytes!("../static/js.js");
const JQUERY_MIN_JS: &[u8] = include_bytes!("../static/cdn/jquery.min.js");

pub fn main() {
    let listener = TcpListener::bind(config::BIND).unwrap();
    eprintln!("server @ http://{}:{}", config::IP, config::PORT);

}
