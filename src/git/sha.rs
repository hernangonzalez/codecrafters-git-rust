use super::DIR_GIT_OBJECTS;
use anyhow::{ensure, Result};
use bytes::{Buf, Bytes};
use sha1::{Digest, Sha1};
use std::{path::Path, str::FromStr};

#[derive(Clone, Debug, PartialEq)]
pub struct Sha(String);

pub const SHA1_CHUNK_SIZE: usize = 20;

impl Sha {
    fn new(inner: String) -> Result<Self> {
        ensure!(inner.len() == 40, "Unexpected sha1 hash");
        Ok(Self(inner))
    }

    pub fn path(&self) -> Box<Path> {
        let (prefix, file_name) = self.0.split_at(2);
        let path = Path::new(DIR_GIT_OBJECTS).join(prefix).join(file_name);
        path.into_boxed_path()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn take_from(source: &mut Bytes) -> Result<Self> {
        ensure!(source.len() >= SHA1_CHUNK_SIZE, "Not enough data");
        let source = source.split_to(SHA1_CHUNK_SIZE);
        source.chunk().try_into()
    }
}

impl FromStr for Sha {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        Self::new(s.to_string())
    }
}

impl TryFrom<&[u8]> for Sha {
    type Error = anyhow::Error;
    fn try_from(chunk: &[u8]) -> Result<Sha> {
        ensure!(chunk.len() == SHA1_CHUNK_SIZE);
        let mut hasher = Sha1::new();
        hasher.update(chunk);
        let result = hasher.finalize();
        let hash = hex::encode(result);
        Sha::new(hash)
    }
}
