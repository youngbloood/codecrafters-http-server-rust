use crate::util::linklist::LinkList;
use anyhow::Error;

#[derive(Debug)]
pub struct PathNode {
    pub raw: String,
    pub key: String,
    pub value: String,
}
pub fn new_path(raw: &str, list: Vec<&str>) -> LinkList<PathNode> {
    let mut ll = LinkList::<PathNode>::new();
    for i in list {
        ll.push(PathNode {
            raw: raw.to_string(),
            key: i.to_string(),
            value: i.to_string(),
        })
    }
    return ll;
}
