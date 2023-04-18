use super::compress;
use super::sha::Sha;
use anyhow::{Context, Result};
use std::fmt::Display;
use std::fs;
use std::io::Write;

const GIT_BLOB_DELIMITER: u8 = b'\x00';
const GIT_KIND_DELIMITER: char = ' ';

pub trait Codable: Sized {
    fn encode(&self) -> Result<(Sha, Vec<u8>)>;
    fn decode(data: Vec<u8>) -> Result<Self>;
}

#[derive(Debug, PartialEq)]
pub enum Kind {
    Blob,
    Tree,
}

#[derive(Debug)]
pub struct Object {
    pub kind: Kind,
    pub data: Vec<u8>,
}

impl Object {
    pub fn blob(data: Vec<u8>) -> Self {
        Self {
            kind: Kind::Blob,
            data,
        }
    }
}

impl Codable for Object {
    fn encode(&self) -> Result<(Sha, Vec<u8>)> {
        let mut data: Vec<u8> = vec![];
        let header = format!("{} {}", self.kind, self.data.len());
        data.write_all(header.as_bytes())?;
        data.push(GIT_BLOB_DELIMITER);
        data.write_all(&self.data)?;

        let hash: Sha = (&data[..]).try_into()?;
        let data = compress::encode(&data)?;
        Ok((hash, data))
    }

    fn decode(data: Vec<u8>) -> Result<Self> {
        let data = compress::decode(&data)?;
        let blob_ix = data
            .iter()
            .enumerate()
            .find(|t| t.1 == &GIT_BLOB_DELIMITER)
            .context("object header")?
            .0;
        let (header, blob) = data.split_at(blob_ix + 1);
        let header = std::str::from_utf8(header)?;
        let (kind, _) = header
            .split_once(GIT_KIND_DELIMITER)
            .context("object kind")?;
        let kind = kind.try_into()?;
        Ok(Self {
            kind,
            data: blob.to_vec(),
        })
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blob => write!(f, "blob"),
            Self::Tree => write!(f, "tree"),
        }
    }
}

impl TryFrom<&str> for Kind {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self> {
        match value {
            "blob" => Ok(Self::Blob),
            "tree" => Ok(Self::Tree),
            k => Err(anyhow::anyhow!("Unknown kind: {k}")),
        }
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
