use super::{compress, GitObject, Kind, Sha, Tree, GIT_BLOB_DELIMITER};
use anyhow::{ensure, Context, Result};
use std::{fs, path::Path};

#[derive(Debug)]
pub struct AnyObject(Kind, Vec<u8>);

impl AnyObject {
    pub fn into_tree(self) -> Result<Tree> {
        ensure!(self.0 == Kind::Tree);
        Ok(Tree::new(self.1))
    }
}

impl GitObject for AnyObject {
    fn kind(&self) -> Kind {
        self.0
    }

    fn bytes(&self) -> &[u8] {
        &self.1
    }
}

impl TryFrom<&Sha> for AnyObject {
    type Error = anyhow::Error;
    fn try_from(sha: &Sha) -> Result<Self> {
        let path: &Path = &sha.path();
        AnyObject::try_from(path)
    }
}

impl TryFrom<&Path> for AnyObject {
    type Error = anyhow::Error;
    fn try_from(path: &Path) -> Result<Self> {
        let blob = fs::read(path).context("read blob")?;
        decode(blob).context("decode blob")
    }
}

fn decode(data: Vec<u8>) -> Result<AnyObject> {
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
    Ok(AnyObject(kind, blob.to_vec()))
}
