mod any;
mod blob;
mod kind;
mod tree;

use super::codec;
use super::sha::Sha;
pub use any::AnyObject;
use anyhow::{Context, Result};
pub use blob::Blob;
use kind::Kind;
use std::io::Write;
pub use tree::{Tree, TreeItem};

const GIT_BLOB_DELIMITER: u8 = b'\x00';
const GIT_KIND_DELIMITER: u8 = b' ';

pub trait GitObject {
    fn kind(&self) -> Kind;
    fn bytes(&self) -> &[u8];

    fn header(&self) -> String {
        let kind = self.kind();
        let len = self.bytes().len();
        format!("{kind}{GIT_KIND_DELIMITER}{len}")
    }

    fn encode(&self) -> Result<(Sha, Vec<u8>)> {
        let header = self.header();
        let bytes = self.bytes();

        let mut data: Vec<u8> = vec![];
        data.write_all(header.as_bytes())?;
        data.push(GIT_BLOB_DELIMITER);
        data.write_all(bytes)?;

        let hash: Sha = (&data[..]).try_into()?;
        let data = codec::zip(&data).context("zip")?;
        Ok((hash, data))
    }
}
