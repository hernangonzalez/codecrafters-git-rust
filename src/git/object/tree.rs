mod builder;
mod scanner;
mod writer;

use crate::git::{codec::Codable, object::tree::scanner::TreeScanner};

use super::{Sha, GIT_BLOB_DELIMITER, GIT_KIND_DELIMITER};
use anyhow::Result;
use builder::TreeBuilder;
use bytes::BytesMut;
use std::path::Path;
use writer::Writer;

#[derive(Debug)]
pub struct TreeItem {
    pub mode: String,
    pub name: String,
    pub sha: Sha,
}

#[derive(Debug)]
pub struct Tree {
    items: Vec<TreeItem>,
}

impl Tree {
    pub fn items(&self) -> &Vec<TreeItem> {
        &self.items
    }

    pub fn read_path(path: &Path) -> Result<Self> {
        let builder = TreeBuilder::new(path);
        builder.build()
    }
}

impl Codable for Tree {
    fn encode(&self, buffer: &mut BytesMut) {
        todo!()
    }

    fn decode(chunk: &[u8]) -> Result<Self> {
        let scanner = TreeScanner { data: chunk };
        let items = scanner.collect();
        Ok(Self { items })
    }
}
