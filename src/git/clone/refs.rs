use crate::git::Sha;
use anyhow::{Context, Result};
use std::str::FromStr;

#[allow(dead_code)]
#[derive(Debug)]
pub struct GitRef {
    pub sha: Sha,
    pub name: String,
    pub size: usize,
}

impl FromStr for GitRef {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let (chunk, name) = s.split_once(' ').context("ref str")?;
        let (size, sha) = chunk.split_at(4);
        let sha = sha.parse::<Sha>()?;
        let name = name.to_string();
        let size = usize::from_str_radix(size, 16)?;
        Ok(Self { sha, name, size })
    }
}
