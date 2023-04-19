use super::{GitObject, Kind};
use anyhow::{ensure, Result};
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Blob(Vec<u8>);

impl TryFrom<&Path> for Blob {
    type Error = anyhow::Error;
    fn try_from(path: &Path) -> Result<Self> {
        ensure!(path.exists());
        let data = fs::read(path)?;
        Ok(Self(data))
    }
}

impl GitObject for Blob {
    fn kind(&self) -> Kind {
        Kind::Blob
    }

    fn bytes(&self) -> &[u8] {
        &self.0
    }
}
