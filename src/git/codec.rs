use super::sha::Sha;
use anyhow::{Context, Result};
use bytes::BytesMut;
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use std::io::{Read, Write};

pub fn unzip(data: &[u8]) -> Result<Vec<u8>> {
    unzip_count(data).map(|x| x.0)
}

pub fn unzip_count(data: &[u8]) -> Result<(Vec<u8>, usize)> {
    let mut decoder = ZlibDecoder::new(data);
    let mut data = vec![];
    decoder.read_to_end(&mut data)?;
    Ok((data, decoder.total_in() as usize))
}

pub fn zip(data: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    let data = encoder.finish()?;
    Ok(data)
}

pub trait Codable: Sized {
    fn encode(&self, buffer: &mut BytesMut) -> Result<()>;
    fn decode(chunk: &[u8]) -> Result<Self>;

    fn encoded(&self) -> Result<BytesMut> {
        let mut buffer = BytesMut::new();
        self.encode(&mut buffer)?;
        Ok(buffer)
    }
}

pub trait Package: Codable {
    fn pack(&self) -> Result<(Sha, Vec<u8>)> {
        let mut buffer = BytesMut::new();
        self.encode(&mut buffer)?;

        let hash: Sha = (&buffer[..]).try_into()?;
        let data = zip(&buffer).context("zip")?;
        Ok((hash, data))
    }

    fn unpack(sha: &Sha) -> Result<Self> {
        let chunk = std::fs::read(sha.path())?;
        let chunk = unzip(&chunk)?;
        Self::decode(&chunk)
    }
}
