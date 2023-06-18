mod item;

use crate::git::sha::{Sha, SHA1_CHUNK_SIZE};
use anyhow::{ensure, Ok, Result};
use bytes::Bytes;

#[allow(dead_code)]
#[derive(Debug)]
pub struct GitPack {
    header: Header,
    checksum: Sha,
}

impl TryFrom<Bytes> for GitPack {
    type Error = anyhow::Error;

    fn try_from(value: Bytes) -> Result<Self> {
        build(value)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Header {
    version: u32,
    count: u32,
}

impl Header {
    const SIZE: usize = 12;
    const GIT_PACK: &[u8] = b"PACK";

    fn build(bytes: &mut Bytes) -> Result<Header> {
        ensure!(bytes.len() >= Self::SIZE);

        let bytes = bytes.split_to(Self::SIZE);
        ensure!(bytes.starts_with(Self::GIT_PACK));

        let version = u32::from_be_bytes(bytes[4..8].try_into()?);
        let count = u32::from_be_bytes(bytes[8..12].try_into()?);

        Ok(Self { version, count })
    }
}

fn build(bytes: Bytes) -> Result<GitPack> {
    let mut bytes = bytes;
    let header = Header::build(&mut bytes)?;

    ensure!(bytes.len() >= SHA1_CHUNK_SIZE);
    let checksum = bytes.split_off(bytes.len() - SHA1_CHUNK_SIZE);
    let checksum: Sha = Sha::try_from(checksum.as_ref())?;

    while !bytes.is_empty() {
        let item = item::read_from(&mut bytes)?;
        let desc = item.desc;
        println!("{:?} - {} - {}", desc.kind, desc.size, bytes.len());
    }

    Ok(GitPack { header, checksum })
}
