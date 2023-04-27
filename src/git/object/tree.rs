mod scanner;

use super::{GIT_BLOB_DELIMITER, GIT_KIND_DELIMITER};
use crate::git::{codec::Codable, object::tree::scanner::TreeScanner, Sha};
use anyhow::{Ok, Result};
use bytes::{BufMut, BytesMut};

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
    pub fn new(items: Vec<TreeItem>) -> Self {
        Self { items }
    }

    pub fn items(&self) -> &Vec<TreeItem> {
        &self.items
    }
}

impl Codable for Tree {
    fn encode(&self, buffer: &mut BytesMut) -> Result<()> {
        for item in &self.items {
            item.encode(buffer)?
        }
        Ok(())
    }

    fn decode(chunk: &[u8]) -> Result<Self> {
        let scanner = TreeScanner { data: chunk };
        let items = scanner.collect();
        Ok(Self { items })
    }
}

impl TreeItem {
    fn encode(&self, buffer: &mut BytesMut) -> Result<()> {
        buffer.put_slice(self.mode.as_bytes());
        buffer.put_u8(GIT_KIND_DELIMITER);
        buffer.put_slice(self.name.as_bytes());
        buffer.put_u8(GIT_BLOB_DELIMITER);

        let sha = hex::decode(self.sha.as_bytes()).unwrap();
        buffer.put_slice(&sha);
        Ok(())
    }
}
