use crate::util::linklist::LinkList;
use anyhow::{anyhow, Error};
use nom::FindSubstring;
use std::cell::RefMut;
use std::path::Path;
use std::rc::Rc;
use std::{collections::HashMap, io::Write, net::TcpStream};
use std::{fs, os, vec};

use super::path::{self, PathNode};

#[derive(Debug)]
pub struct Http {
    pub method: String,
    pub full_path: String,
    pub path: Option<LinkList<PathNode>>,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Http {
    pub fn new() -> Self {
        let http = Self {
            method: "".to_string(),
            full_path: "".to_string(),
            path: None,
            http_version: "".to_string(),
            headers: HashMap::<String, String>::new(),
            body: vec![],
        };
        return http;
    }

    // 寻找第一个substr位置
    fn find_first_blank_pos(raw: &[u8], substr: &str) -> usize {
        match raw.find_substring(substr) {
            Some(pos) => return pos,
            None => return 0,
        }
    }

    // 解析http基本信息，包含Method，请求路径，http协议版本
    // 解析http的header信息
    // return Content-Length
    pub fn parse_base(&mut self, mut raw: &[u8]) -> Result<(), Error> {
        let blank_pos = Self::find_first_blank_pos(raw, "\r\n\r\n");
        if blank_pos == 0 {
            return Ok(());
        }

        let (main, mut left) = raw.split_at(blank_pos);
        // raw = &mut left;

        let main_str = std::str::from_utf8(&main).unwrap();
        // 解析http基本信息，包含Method，请求路径，http协议版本
        let (base, headers) = main_str.split_at(main_str.find_substring("\r\n").unwrap_or(0));
        let cells: Vec<&str> = base.splitn(3, ' ').collect();

        self.method = cells[0].to_string();
        self.full_path = cells[1].to_string();
        self.path = Some(path::new_path(
            &self.full_path,
            self.full_path.split("/").collect(),
        ));
        self.http_version = cells[2].to_string();

        // 解析http的header信息
        let header_pairs = headers.trim().split("\r\n");
        for line in header_pairs {
            if line.len() == 0 {
                continue;
            }
            let mut kvs = line.splitn(2, |x| x == ':');
            self.headers.insert(
                kvs.next().unwrap().to_owned(),
                kvs.next().unwrap().trim().to_owned(),
            );
        }

        self.body = raw[blank_pos + 4..].to_vec();
        println!("http = {:?}", self);

        self.handle_body()?;
        Ok(())
    }

    fn handle_body(&mut self) -> Result<(), Error> {
        let filename = &self.path.as_ref().unwrap().index(1).as_ref().unwrap().value;
        let save_dir = Path::new("./target");
        let binding = save_dir.join(filename);
        let filepath = binding.as_path();

        match self.headers().get("Content-Type") {
            Some(v) => match v.as_str() {
                // save the markdown
                "text/markdown" => Ok({
                    let mut fd = fs::File::create(filepath).unwrap();
                    fd.write_all(&self.body)?;
                }),
                _ => return Err(anyhow!("Not Support Content-Type: {}", v.as_str())),
            },
            None => return Ok(()),
        }
    }

    pub fn method(&self) -> &str {
        return self.method.as_str();
    }
    pub fn path(&self) -> &str {
        return self.full_path.as_str();
    }
    pub fn http_version(&self) -> &str {
        return self.http_version.as_str();
    }
    pub fn headers(&self) -> HashMap<String, String> {
        return self.headers.clone();
    }
}
