use crate::git::{codec, object::Kind, Sha};
use anyhow::{Context, Ok, Result};
use bytes::{Buf, Bytes};
use std::mem::size_of;

pub type GitFileSize = u32;

#[allow(dead_code)]
#[derive(Debug)]
pub struct PackItemDescriptor {
    pub kind: PackKind,
    pub size: GitFileSize,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct PackItem {
    pub desc: PackItemDescriptor,
    pub data: Vec<u8>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum PackKind {
    Object(Kind),
    Ref(Sha),
    Offset(i32),
}

pub fn read_from(bytes: &mut Bytes) -> Result<PackItem> {
    let desc = read_desc(bytes)?;
    let (data, read) = codec::unzip_count(bytes)?;
    bytes.advance(read);
    Ok(PackItem { desc, data })
}

fn resolve_kind(raw: u8, source: &mut Bytes) -> Result<PackKind> {
    let kind = match raw {
        1 => PackKind::Object(Kind::Commit),
        2 => PackKind::Object(Kind::Tree),
        3 => PackKind::Object(Kind::Blob),
        7 => {
            let sha = Sha::take_from(source)?;
            PackKind::Ref(sha)
        }
        _ => return Err(anyhow::anyhow!("Pack Item not supported")),
    };
    Ok(kind)
}

fn read_desc(source: &mut Bytes) -> Result<PackItemDescriptor> {
    const PARTIAL_MASK: u8 = 0b10000000;
    const SIZE_LIMIT: usize = size_of::<GitFileSize>();
    const KIND_MASK: u8 = 0b01110000;

    let mut stream = source.iter();
    let is_last_chunk = |x| (x & PARTIAL_MASK) == 0;

    let mut value: GitFileSize = 0;
    let mut raw_kind = 0;
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
            raw_kind = (part & KIND_MASK) >> 4;
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
    source.advance(bytes_read);

    let kind = resolve_kind(raw_kind, source)?;
    let size = u32::from_le(value);
    let desc = PackItemDescriptor { kind, size };
    Ok(desc)
}
