use crate::git::{
    codec::{Codable, Package},
    object::{Body, Header, Kind, Object, ObjectBuilder, Tree, TreeItem},
};
use anyhow::{Context, Result};
use bytes::BytesMut;
use std::fs;
use std::path::Path;

const GIT_MODE_FILE: &str = "100644";
const GIT_MODE_FOLDER: &str = "40000";

pub fn tree_at_path(path: &Path, skip: &[&str]) -> Result<Object> {
    let entries = path.read_dir()?;
    let mut items: Vec<TreeItem> = Vec::with_capacity(entries.count());

    for entry in path.read_dir()? {
        let entry = entry?;
        let ftype = entry.file_type()?;
        let path = entry.path();
        let obj: Object;
        let mode: &str;

        if skip.contains(&entry.file_name().to_str().context("entry name")?) {
            continue;
        }

        if ftype.is_file() {
            let data = fs::read(path.clone())?;
            let builder = ObjectBuilder::blob(&data);
            obj = builder.build()?;
            mode = GIT_MODE_FILE;
        } else if ftype.is_dir() {
            obj = tree_at_path(&path, skip)?;
            mode = GIT_MODE_FOLDER;
        } else {
            continue;
        }

        let (sha, _) = obj.pack()?;
        let mode = mode.to_string();
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string())
            .unwrap_or_default();
        let item = TreeItem { mode, name, sha };
        items.push(item);
    }

    items.sort_by(|l, r| l.name.cmp(&r.name));
    let tree = Tree::new(items);
    let mut buffer = BytesMut::new();
    tree.encode(&mut buffer)?;

    let body = Body::Tree(tree);
    let header = Header::new(Kind::Tree, buffer.len());
    let obj = Object::new(header, body);
    Ok(obj)
}
