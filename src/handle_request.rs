use crate::http::http::Http;
use anyhow::Error;
use nom::{AsBytes, FindSubstring};
use regex::Regex;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

const READ_LENGH: usize = 1024;

pub fn handle_tcp(ts: &mut TcpStream) -> Result<(), Error> {
    // 读取所有的数据至all中
    let mut all = Vec::<u8>::new();
    loop {
        let mut buf = [0u8; READ_LENGH];
        let actual_len = ts.read(&mut buf).unwrap();

        if actual_len == 0 || actual_len < READ_LENGH {
            all.extend(&buf[..actual_len].to_vec());
            break;
        }
        all.extend(&buf.to_vec());
    }

    let mut htp = Http::new();
    htp.parse_base(&all)?;

    // GET: /abc/def
    // RESP: def
    resp_last_path(ts, &htp);

    // GET: /abc/def User-Agent: <USER-AGENT>
    // RESP: <USER-AGENT>
    // get_user_agent(ts, &htp);

    // GET: /files/main.rs
    // RESP: main.rs's content
    // get_file(ts, &htp);

    // POST: /files/<SAVED-FILENAME>
    // post_file(ts, &htp);

    ts.shutdown(std::net::Shutdown::Both)?;

    return Ok(());
}

// 返回header中的User-Agent的值
pub fn get_user_agent(ts: &mut TcpStream, htp: &Http) {
    let _user_agent_path = &"/user-agent".to_string();
    let resp_tmpl = match &htp.full_path {
        _user_agent_path => {
            Some("HTTP/1.1 200 OK\r\n\r\nContent-Type: text/plain\r\nContent-Length: ")
        }
        _ => None,
    };
    if resp_tmpl.is_none() {
        return;
    }
    let headers = htp.headers();
    let default_agent = "".to_string();
    let user_agent = headers.get("User-Agent").unwrap_or(&default_agent);
    let mut resp = format!("{}{}", resp_tmpl.unwrap(), user_agent.len());
    resp = format!("{}\r\n\r\n{}", resp, user_agent);

    ts.write(resp.as_bytes()).expect("write tcpstream failed");
}

// 返回请求路径的最后一个
// eg： GET /abc/edf
// 返回 edf
pub fn resp_last_path(ts: &mut TcpStream, htp: &Http) {
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
    ts.write(response.as_bytes())
        .expect("Failed to write response bytes to stream");
}

// 获取一个文件
pub fn get_file(ts: &mut TcpStream, htp: &Http) {
    let path = htp.path.as_ref();
    if htp.method() != "GET"
        || path.is_none()
        || path.unwrap().len() != 2
        || path.unwrap().index(0).unwrap().value != "files"
    {
        ts.write("HTTP/1.1 404 NOT FOUND".as_bytes())
            .expect("write tcpstream failed");
        return;
    }

    let filename = &path.unwrap().index(path.unwrap().len() - 1).unwrap().value;
    let mut read_dir = std::fs::read_dir("./src").unwrap();
    match read_dir
        .find(|x| {
            return x.as_ref().unwrap().file_name().into_string().unwrap() == filename.to_string();
        })
        .unwrap()
    {
        // 找到该文件，读取并返回
        Ok(filename) => {
            let content = std::fs::read(filename.path()).unwrap();
            let resp = format!(
                "HTTP/1.1 200 OK
Content-Type: application/octet-stream
Content-Disposition: attachment; filename={file_name:?}
Content-Length: {content_len}


{content}",
                content = String::from_utf8_lossy(&content),
                content_len = content.len(),
                file_name = filename.file_name(),
            );

            ts.write(resp.as_bytes()).expect("write tcpstream failed");
            return;
        }
        Err(e) => {
            eprint!("Error: {}", e);
            ts.write("HTTP/1.1 404 NOT FOUND".as_bytes())
                .expect("write tcpstream failed");
            return;
        }
    };
}

fn post_file(ts: &mut TcpStream, htp: &Http) {
    let path = htp.path.as_ref();
    if htp.method() != "GET"
        || path.is_none()
        || path.unwrap().len() != 2
        || path.unwrap().index(0).unwrap().value != "files"
    {
        ts.write("HTTP/1.1 404 NOT FOUND".as_bytes())
            .expect("write tcpstream failed");
        return;
    }

    ts.write("HTTP/1.1 200 OK".as_bytes())
        .expect("write tcpstream failed");
    return;
}
