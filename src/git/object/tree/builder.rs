use super::{TreeItem, Writer};
use crate::git::{Blob, Sha, Tree};
use anyhow::{Context, Result};
use std::path::Path;

pub struct TreeBuilder<'a>(&'a Path);

impl<'a> TreeBuilder<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self(path)
    }

    pub fn build(&self) -> Result<Tree> {
        let path = self.0;
        let entries = path.read_dir()?;
        let mut items: Vec<TreeItem> = Vec::with_capacity(entries.count());

        todo!()

        // for entry in path.read_dir()? {
        //     let entry = entry?;
        //     let ftype = entry.file_type()?;
        //     let path = entry.path();
        //     let mut sha: Option<Sha> = None;

        //     if ftype.is_file() {
        //         let obj = Blob::try_from(path.as_path())?;
        //         sha = Some(obj.encode()?.0);
        //     } else if ftype.is_dir() {
        //         let path = entry.path();
        //         let tree = Tree::read_path(&path)?;
        //         sha = Some(tree.encode()?.0);
        //     }

        //     let name = path
        //         .file_name()
        //         .and_then(|s| s.to_str())
        //         .map(|s| s.to_string())
        //         .context("file name")?;
        //     let mode = "111".to_string();
        //     let item = TreeItem {
        //         mode,
        //         name,
        //         sha: sha.unwrap(),
        //     };
        //     items.push(item);
        // }

        // let writer = Writer { items: &items };
        // let blob = writer.to_bytes();
        // let tree = Tree::new(blob.to_vec());
        // Ok(tree)
    }
}
