mod blob;
mod kind;
mod tree;

use super::compress;
use super::sha::Sha;
use anyhow::{ensure, Context, Result};
pub use blob::Blob;
use kind::Kind;
use std::fs;
use std::io::Write;
pub use tree::Tree;

const GIT_BLOB_DELIMITER: u8 = b'\x00';
const GIT_KIND_DELIMITER: u8 = b' ';

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

#[derive(Debug)]
pub struct Object(Kind, Vec<u8>);

impl Object {
    pub fn into_tree(self) -> Result<Tree> {
        ensure!(self.0 == Kind::Tree);
        Ok(Tree::new(self.1))
    }

    pub fn bytes(self) -> Vec<u8> {
        self.1
    }

    pub fn blob(data: Vec<u8>) -> Self {
        Self(Kind::Blob, data)
    }
}

impl Object {
    pub fn encode(&self) -> Result<(Sha, Vec<u8>)> {
        let header = format!("{} {}", self.0, self.1.len());
        let mut data: Vec<u8> = vec![];
        data.write_all(header.as_bytes())?;
        data.push(GIT_BLOB_DELIMITER);
        data.write_all(&self.1)?;

        let hash: Sha = (&data[..]).try_into()?;
        let data = compress::encode(&data)?;
        Ok((hash, data))
    }

    pub fn decode(data: Vec<u8>) -> Result<Object> {
        let data = compress::decode(&data)?;
        let blob_ix = data
            .iter()
            .enumerate()
            .find(|t| t.1 == &GIT_BLOB_DELIMITER)
            .context("object header")?
            .0;
        let (header, blob) = data.split_at(blob_ix + 1);
        let header = std::str::from_utf8(header)?;
        let (kind, _) = header.split_once(' ').context("object kind")?; // fixme!
        let kind = kind.try_into()?;
        Ok(Object(kind, blob.to_vec()))
    }
}

impl TryFrom<&Sha> for Object {
    type Error = anyhow::Error;
    fn try_from(sha: &Sha) -> Result<Self> {
        let path = sha.path();
        let blob = fs::read(path).context("read blob")?;
        Object::decode(blob).context("decode blob")
    }
}
