mod http;
mod util;

use crate::http::http::Http;
use anyhow::Error;
use nom::AsBytes;
use std::fmt::format;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::vec;

fn main() -> Result<(), Error> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => handle_tcp(&mut stream)?,
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    return Ok(());
}

fn handle_tcp(ts: &mut TcpStream) -> Result<(), Error> {
    ts.write_all(b"HTTP/1.1 200 OK\r\n\r\n")
        .expect("write failed");

    let mut buf = [0u8; 256];
    ts.read(&mut buf)?;

    let raw_req = std::str::from_utf8(&buf).unwrap().trim_end_matches("\0");
    println!("raw = {:?}", raw_req);

    let htp = Http::new(raw_req)?;

    let resp_tmpl = match htp.path() {
        _ => "HTTP/1.1 200 OK\r\n\r\nContent-Type: text/plain\r\nContent-Length: ",
    };

    let last = htp
        .path
        .as_ref()
        .unwrap()
        .index(htp.path.as_ref().unwrap().len() - 1)
        .unwrap()
        .value
        .as_str();

    let mut response = format!("{} {}", String::from(resp_tmpl), last.len());
    response = format!("{}\r\n\r\n{}", response, last);
    println!("response = {}", response);
    ts.write(response.as_bytes())
        .expect("Failed to write response bytes to stream");

    ts.shutdown(std::net::Shutdown::Both)?;

    return Ok(());
}
