use super::compress;
use super::sha::Sha;
use super::GIT_BLOB_DELIMITER;
use anyhow::Result;
use std::fmt::Display;
use std::io::Write;

pub enum Kind {
    Blob,
}

pub struct Object {
    kind: Kind,
    data: Vec<u8>,
}

impl Object {
    pub fn blob(data: Vec<u8>) -> Self {
        Self {
            kind: Kind::Blob,
            data,
        }
    }

    pub fn encode(&self) -> Result<(Sha, Vec<u8>)> {
        let mut data: Vec<u8> = vec![];
        let header = format!("{} {}", self.kind, self.data.len());
        data.write_all(header.as_bytes())?;
        data.push(GIT_BLOB_DELIMITER);
        data.write_all(&self.data)?;

        let hash: Sha = (&data[..]).try_into()?;
        let data = compress::encode(&data)?;
        Ok((hash, data))
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blob => write!(f, "blob"),
        }
    }
}
