use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                handle_tcpstream(&mut _stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_tcpstream(ts: &mut TcpStream) {
    let mut buffer = Vec::new();
    // read the whole file
    let _ = ts.read(&mut buffer);
    // println!("buf = {:?}", buffer);
    let resp = String::from("HTTP/1.1 200 OK\r\n\r\n");
    let _ = ts.write_all(resp.as_bytes()).expect("write failed");
}
