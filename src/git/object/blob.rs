use crate::git::codec::Codable;
use anyhow::Result;
use bytes::{BufMut, BytesMut};

#[derive(Debug)]
pub struct Blob(Vec<u8>);

impl<'a> Blob {
    pub fn as_bytes(&'a self) -> &'a [u8] {
        &self.0
    }
}

impl Codable for Blob {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_slice(&self.0)
    }

    fn decode(chunk: &[u8]) -> Result<Self> {
        Ok(Self(chunk.to_vec()))
    }
}
