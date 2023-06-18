use crate::git::{codec::Codable, Sha};
use anyhow::Result;
use bytes::BufMut;

#[allow(dead_code)]
#[derive(PartialEq, Clone, Debug)]
pub enum Kind {
    Blob,
    Tree,
    Commit,
    Tag,
    ObjectRef(Sha),
    ObjectOffet(i32),
}

impl Codable for Kind {
    fn encode(&self, buffer: &mut bytes::BytesMut) -> Result<()> {
        match self {
            Self::Blob => buffer.put_slice(b"blob"),
            Self::Tree => buffer.put_slice(b"tree"),
            Self::Commit => buffer.put_slice(b"commit"),
            _ => return Err(anyhow::anyhow!("not implemented")),
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
