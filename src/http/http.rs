use crate::util::linklist::LinkList;
use anyhow::Error;
use std::cell::RefMut;
use std::rc::Rc;
use std::{collections::HashMap, io::Write, net::TcpStream};

use super::path::{self, PathNode};

#[derive(Debug)]
pub struct Http {
    pub raw: String,
    pub method: String,
    pub full_path: String,
    pub path: Option<LinkList<PathNode>>,
    pub http_version: String,
    pub headers: HashMap<String, String>,
}

impl Http {
    pub fn new(str: &str) -> Result<Self, Error> {
        let mut http = Self {
            raw: str.to_string(),
            method: "".to_string(),
            full_path: "".to_string(),
            path: None,
            http_version: "".to_string(),
            headers: HashMap::<String, String>::new(),
        };
        http.parse()?;
        return Ok(http);
    }

    fn parse(&mut self) -> Result<(), Error> {
        let mut list = self.raw.split("\r\n");

        // 解析基本的method,path,httpversion
        let mut base = list.next().unwrap().split(" ");
        self.method = base.next().unwrap().to_owned();

        // path的解析
        self.full_path = base.next().unwrap().to_owned();
        self.path = Some(path::new_path(
            &self.full_path,
            self.full_path.split("/").collect(),
        ));
        self.http_version = base.next().unwrap().to_owned();

        // 解析headers
        let mut line = list.next().unwrap();
        while line.len() != 0 {
            let mut kvs = line.splitn(2, |x| x == ':');
            self.headers.insert(
                kvs.next().unwrap().to_owned(),
                kvs.next().unwrap().to_owned(),
            );
            if list.next().is_none() {
                break;
            }
            line = list.next().unwrap();
        }

        // TODO: 解析body
        let content_type = match self.headers.get("Content-Type") {
            Some(ct) => ct,
            None => "",
        };

        println!("http = {:?}", self);

        Ok(())
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
