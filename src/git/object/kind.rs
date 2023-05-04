use crate::git::codec::Codable;
use anyhow::Result;
use bytes::BufMut;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Kind {
    Blob,
    Tree,
    Commit,
}

impl Codable for Kind {
    fn encode(&self, buffer: &mut bytes::BytesMut) -> Result<()> {
        match self {
            Self::Blob => buffer.put_slice(b"blob"),
            Self::Tree => buffer.put_slice(b"tree"),
            Self::Commit => buffer.put_slice(b"commit"),
        };
        Ok(())
    }

    fn decode(chunk: &[u8]) -> Result<Self> {
        match chunk {
            b"blob" => Ok(Self::Blob),
            b"tree" => Ok(Self::Tree),
            b"commit" => Ok(Self::Commit),
            k => Err(anyhow::anyhow!("Unknown kind: {k:?}")),
        }
    }
}
