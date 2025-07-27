#![allow(dead_code)]
#![allow(unused_variables)]

mod config;

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

// HTTP return codes
const HTTP_200_OK: &[u8] = b"HTTP/1.1 200 OK\r\n";
const HTTP_404_NOTFOUND: &[u8] = b"HTTP/1.1 404 Not Found\r\n";

// MIME types
const TEXT_HTML: &[u8] = b"Content-Type: text/html; charset=utf-8\r\n";
const TEXT_PLAIN: &[u8] = b"Content-Type: text/plain; charset=utf-8\r\n";
const TEXT_CSS: &[u8] = b"Content-Type: text/css\r\n";
const TEXT_JS: &[u8] = b"Content-Type: application/javascript\r\n";
const IMAGE_PNG: &[u8] = b"Content-Type: image/png\r\n";
const WASM_MODULE: &[u8] = b"Content-Type: application/wasm\r\n";

// jigs
const HTTP_CACHE: &[u8] = b"Cache-Control: public, max-age=3600\r\n";
const HTTP_IMMUTABLE: &[u8] = b"Cache-Control: immutable, max-age=3600\r\n";
const HTTP_NOCACHE: &[u8] = b"Cache-Control: no-cache\r\n";

// static content
const INDEX_HEAD: &[u8] = include_bytes!("../static/head.html");
const INDEX_BODY: &[u8] = include_bytes!("../static/index.html");
const INDEX_FOOT: &[u8] = include_bytes!("../static/foot.html");
const LOGO_PNG: &[u8] = include_bytes!("../doc/logo.png");
const CSS_CSS: &[u8] = include_bytes!("../static/css.css");
const JS_JS: &[u8] = include_bytes!("../static/js.js");
const JQUERY_MIN_JS: &[u8] = include_bytes!("../static/cdn/jquery.min.js");

// WASM modules
const HELLO_WASM: &[u8] = include_bytes!("../static/hello.wasm");

fn error_404(client: &mut TcpStream, request: &[u8]) {
    client.write(&HTTP_404_NOTFOUND).unwrap();
    client.write(&TEXT_HTML).unwrap();
    client.write(&HTTP_NOCACHE).unwrap();
    client.write(b"\r\n").unwrap();
    client.write(&INDEX_HEAD).unwrap();
    client.write(b"<pre>\r\n").unwrap();
    client.write(request).unwrap();
    client.write(b"</pre></body></html>\r\n").unwrap();
}

fn html(client: &mut TcpStream, body: &[u8]) {
    client.write(&HTTP_200_OK).unwrap();
    client.write(&TEXT_HTML).unwrap();
    client.write(&HTTP_CACHE).unwrap();
    client.write(b"\r\n").unwrap();
    client.write(&INDEX_HEAD).unwrap();
    client.write(body).unwrap();
    client.write(&INDEX_FOOT).unwrap();
    client.flush().unwrap();
}

fn logo(client: &mut TcpStream) {
    client.write(&HTTP_200_OK).unwrap();
    client.write(&IMAGE_PNG).unwrap();
    client.write(&HTTP_IMMUTABLE).unwrap();
    client.write(b"\r\n").unwrap();
    client.write(&LOGO_PNG).unwrap();
    client.flush().unwrap();
}

fn css(client: &mut TcpStream) {
    client.write(&HTTP_200_OK).unwrap();
    client.write(&TEXT_CSS).unwrap();
    client.write(&HTTP_CACHE).unwrap();
    client.write(b"\r\n").unwrap();
    client.write(&CSS_CSS).unwrap();
    client.flush().unwrap();
}

fn js(client: &mut TcpStream, cache: &[u8], body: &[u8]) {
    client.write(&HTTP_200_OK).unwrap();
    client.write(&TEXT_JS).unwrap();
    client.write(cache).unwrap();
    client.write(b"\r\n").unwrap();
    client.write(body).unwrap();
    client.flush().unwrap();
}

fn wasm(client: &mut TcpStream, body: &[u8]) {
    client.write(&HTTP_200_OK).unwrap();
    client.write(&WASM_MODULE).unwrap();
    client.write(&HTTP_NOCACHE).unwrap();
    client.write(b"\r\n").unwrap();
    client.write(body).unwrap();
    client.flush().unwrap();
}

fn router(client: &mut TcpStream) {
    let mut request = [0; 1024];
    client.read(&mut request).unwrap();

    let methurl = request.split(|&x| x == b'\n').next().unwrap();
    let parts: Vec<&[u8]> = methurl.split(|&x| x == b' ').collect();
    let (method, url) = (parts[0], parts[1]);

    match (method, url) {
        (b"GET", b"/") | (b"GET", b"/index.html") => html(client, INDEX_BODY),
        (b"GET", b"/favicon.ico") | (b"GET", b"/logo.png") => logo(client),
        (b"GET", b"/css.css") => css(client),
        (b"GET", b"/jquery.min.js") => js(client, HTTP_IMMUTABLE, JQUERY_MIN_JS),
        (b"GET", b"/js.js") => js(client, HTTP_CACHE, JS_JS),
        (b"GET", b"/hello.wasm") => wasm(client, HELLO_WASM),
        _ => error_404(client, &request),
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
