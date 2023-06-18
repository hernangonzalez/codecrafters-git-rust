use super::DIR_GIT_OBJECTS;
use anyhow::{ensure, Result};
use sha1::{Digest, Sha1};
use std::{path::Path, str::FromStr};

#[derive(Clone, Debug, PartialEq)]
pub struct Sha(String);

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
        let mut hasher = Sha1::new();
        hasher.update(chunk);
        let result = hasher.finalize();
        let hash = hex::encode(result);
        Sha::new(hash)
    }
}
