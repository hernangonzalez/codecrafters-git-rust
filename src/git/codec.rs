use anyhow::Result;
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use std::io::{Read, Write};

pub fn zip(data: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(data);
    let mut data = vec![];
    decoder.read_to_end(&mut data)?;
    Ok(data)
}

pub fn unzip(data: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    let data = encoder.finish()?;
    Ok(data)
}
