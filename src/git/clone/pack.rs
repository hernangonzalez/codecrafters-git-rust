use anyhow::{ensure, Result};
use bytes::Bytes;

#[allow(dead_code)]
#[derive(Debug)]
pub struct GitPack {
    header: Header,
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

    // Header
    let header = Header::build(&mut bytes)?;
    dbg!(&header);

    Ok(GitPack { header })
}
