use std::mem::size_of;

use crate::git::{codec, object::Kind, Sha};
use anyhow::{ensure, Context, Ok, Result};
use bytes::{Buf, Bytes};

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

    while !bytes.is_empty() {
        let item = read_item(&mut bytes)?;
        let desc = item.desc;
        println!("{:?} - {} - {}", desc.kind, desc.size, bytes.len());
    }

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

#[allow(dead_code)]
#[derive(Debug)]
struct PackItem {
    desc: PackItemDescriptor,
    data: Vec<u8>,
}

fn read_item(bytes: &mut Bytes) -> Result<PackItem> {
    let (desc, read) = read_desc(bytes)?;
    bytes.advance(read);

    let (data, read) = codec::unzip_count(bytes)?;
    bytes.advance(read);

    Ok(PackItem { desc, data })
}

fn read_desc(source: &[u8]) -> Result<(PackItemDescriptor, usize)> {
    const PARTIAL_MASK: u8 = 0b10000000;
    const KIND_MASK: u8 = 0b01110000;
    const SIZE_LIMIT: usize = size_of::<GitFileSize>();

    let mut stream = source.iter();
    let is_last_chunk = |x| (x & PARTIAL_MASK) == 0;
    let resolve_kind = |x| {
        let raw_kind: u8 = (x & KIND_MASK) >> 4;
        match raw_kind {
            1 => Some(Kind::Commit),
            2 => Some(Kind::Tree),
            3 => Some(Kind::Blob),
            _ => None, // unsupported
        }
    };

    let mut value: GitFileSize = 0;
    let mut kind = None;
    let mut bit_read_count = 0;
    let mut collected = false;
    let mut bytes_read = 0;

    while !collected && bit_read_count < (SIZE_LIMIT - 1) * 8 {
        let mut part = *stream.next().context("byte")?;
        collected = is_last_chunk(part);
        let bit_offset: usize;

        // First byte is special
        // Contains [partial:1] + [kind:3] + [data:4]
        if bit_read_count == 0 {
            kind = resolve_kind(part);
            part &= !KIND_MASK;
            bit_offset = 4;
        }
        // Remainings are [partial:1] + [data:7]
        else {
            bit_offset = 7;
        }

        // Remove the MSB flag
        part &= !PARTIAL_MASK;

        // Shift data to the target offset
        let mut data = part as GitFileSize;
        data <<= bit_read_count;

        // Copy bits into collected value
        value |= data;
        bit_read_count += bit_offset;
        bytes_read += 1;
    }

    let desc = PackItemDescriptor {
        kind: kind.context("kind")?,
        size: u32::from_le(value),
    };

    Ok((desc, bytes_read))
}
