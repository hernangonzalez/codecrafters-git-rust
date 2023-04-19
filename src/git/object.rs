mod any;
mod blob;
mod kind;
mod tree;

use super::compress;
use super::sha::Sha;
pub use any::AnyObject;
use anyhow::Result;
pub use blob::Blob;
use kind::Kind;
use std::io::Write;
pub use tree::Tree;

const GIT_BLOB_DELIMITER: u8 = b'\x00';
const GIT_KIND_DELIMITER: u8 = b' ';

pub trait GitObject: Sized {
    fn kind(&self) -> Kind;
    fn bytes(&self) -> &[u8];

    fn header(&self) -> String {
        let kind = self.kind();
        let len = self.bytes().len();
        format!("{kind} {len}")
    }

    fn encode(&self) -> Result<(Sha, Vec<u8>)> {
        let header = self.header();
        let bytes = self.bytes();

        let mut data: Vec<u8> = vec![];
        data.write_all(header.as_bytes())?;
        data.push(GIT_BLOB_DELIMITER);
        data.write_all(bytes)?;

        let hash: Sha = (&data[..]).try_into()?;
        let data = compress::encode(&data)?;
        Ok((hash, data))
    }
}
