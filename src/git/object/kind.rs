use crate::git::codec::Codable;
use anyhow::Result;
use bytes::BufMut;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Kind {
    Blob,
    Tree,
}

impl Codable for Kind {
    fn encode(&self, buffer: &mut bytes::BytesMut) {
        match self {
            Self::Blob => buffer.put_slice(b"blob"),
            Self::Tree => buffer.put_slice(b"tree"),
        }
    }

    fn decode(chunk: &[u8]) -> Result<Self> {
        match chunk {
            b"blob" => Ok(Self::Blob),
            b"tree" => Ok(Self::Tree),
            k => Err(anyhow::anyhow!("Unknown kind: {k:?}")),
        }
    }
}
