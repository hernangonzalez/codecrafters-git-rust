mod builder;
mod scanner;
mod writer;

use super::{GitObject, Kind, Sha, GIT_BLOB_DELIMITER, GIT_KIND_DELIMITER};
use anyhow::Result;
use builder::TreeBuilder;
use scanner::TreeScanner;
use std::path::Path;
use writer::Writer;

#[derive(Debug)]
pub struct TreeItem {
    pub mode: String,
    pub name: String,
    pub sha: Sha,
}

#[derive(Debug)]
pub struct Tree(Vec<u8>);

impl GitObject for Tree {
    fn kind(&self) -> Kind {
        Kind::Tree
    }

    fn bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Tree {
    pub fn new(inner: Vec<u8>) -> Self {
        Self(inner)
    }

    pub fn items(&self) -> Vec<TreeItem> {
        let scanner = TreeScanner { data: &self.0 };
        scanner.collect()
    }

    pub fn read_path(path: &Path) -> Result<Self> {
        let builder = TreeBuilder::new(path);
        builder.build()
    }
}
