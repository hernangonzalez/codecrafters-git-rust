use std::mem::size_of;

use crate::git::{object::Kind, Sha};
use anyhow::{ensure, Context, Result};
use bytes::Bytes;

const CHECKSUM_SIZE: usize = 20;
type GitFileSize = u32;

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

    ensure!(bytes.len() >= CHECKSUM_SIZE);
    let checksum = bytes.split_off(bytes.len() - CHECKSUM_SIZE);
    let checksum: Sha = Sha::try_from(&checksum[..])?;

    let size = file_size(&mut bytes.iter())?;
    dbg!(size);

    // let files = PackFileList { chunk: bytes };
    // for file in files {
    //     dbg!(file);
    // }

    Ok(GitPack { header, checksum })
}

struct PackFileList {
    _chunk: Bytes,
}

impl Iterator for PackFileList {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct PackItemDescriptor {
    kind: Kind,
    size: GitFileSize,
}

fn file_size<'a>(stream: &mut impl Iterator<Item = &'a u8>) -> Result<PackItemDescriptor> {
    const PARTIAL_MASK: u8 = 0b10000000;
    const KIND_MASK: u8 = 0b01110000;
    const SIZE_LIMIT: usize = size_of::<GitFileSize>();

    let is_last_chunk = |x| (x & PARTIAL_MASK) == 0;
    let resolve_kind = |x| {
        let raw_kind = x & KIND_MASK >> 4;
        match raw_kind {
            1 => Some(Kind::Commit),
            2 => Some(Kind::Tree),
            3 => Some(Kind::Blob),
            _ => None, // unsupported
        }
    };

    let mut parts: [u8; SIZE_LIMIT] = [0; SIZE_LIMIT];
    let mut kind = None;

    for (ix, size_byte) in parts.iter_mut().enumerate() {
        let mut part = *stream.next().context("size part")?;
        let collected = is_last_chunk(part);

        if ix == 0 {
            kind = resolve_kind(part);
            part &= !KIND_MASK;
        }

        part &= !PARTIAL_MASK;
        *size_byte = part;

        if collected {
            break;
        } else if ix == SIZE_LIMIT - 1 {
            return Err(anyhow::anyhow!("Not enough buffer to capture file size"));
        }
    }

    Ok(PackItemDescriptor {
        kind: kind.context("kind")?,
        size: GitFileSize::from_le_bytes(parts),
    })
}
