use anyhow::{ensure, Context, Result};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::{
    fs,
    io::{self, Read, Write},
    path::Path,
};

const DIR_GIT: &str = ".git";
const DIR_GIT_OBJECTS: &str = ".git/objects";
const DIR_GIT_REFS: &str = ".git/refs";
const GIT_BLOB_DELIMITER: u8 = b'\x00';

pub fn init() -> Result<()> {
    fs::create_dir(DIR_GIT)?;
    fs::create_dir(DIR_GIT_OBJECTS)?;
    fs::create_dir(DIR_GIT_REFS)?;
    fs::write(".git/HEAD", "ref: refs/heads/master\n")?;
    Ok(())
}

pub fn cat_file(sha: &str) -> Result<()> {
    let path = path_from_sha1(sha)?;
    let data = fs::read(path)?;

    let mut decoder = ZlibDecoder::new(&data[..]);
    let mut data = vec![];
    decoder.read_to_end(&mut data)?;

    let blob_ix = data
        .iter()
        .enumerate()
        .find(|t| t.1 == &GIT_BLOB_DELIMITER)
        .context("")?
        .0;
    let (_, blob) = data.split_at(blob_ix + 1);

    io::stdout().write_all(blob)?;
    Ok(())
}

fn path_from_sha1(hash: &str) -> Result<Box<Path>> {
    ensure!(hash.len() == 40, "Unexpected sha1 hash");
    let (prefix, file_name) = hash.split_at(2);

    let path = Path::new(DIR_GIT_OBJECTS).join(prefix).join(file_name);
    Ok(path.into_boxed_path())
}

fn encode(data: Vec<u8>) -> Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&data)?;
    let data = encoder.finish()?;
    Ok(data)
}

fn sha1(data: &[u8]) -> Result<String> {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    let hash = hex::encode(result);
    Ok(hash)
}

pub fn hash_object(filename: &str) -> Result<()> {
    let path = Path::new(filename);
    ensure!(path.exists());

    let blob = fs::read(path)?;

    let mut data: Vec<u8> = vec![];
    let header = format!("blob {}", blob.len());
    data.write_all(header.as_bytes())?;
    data.push(GIT_BLOB_DELIMITER);
    data.write_all(&blob)?;

    let hash = sha1(&data[..])?;
    let path = path_from_sha1(&hash)?;
    let dir = path.parent().context("blob dir")?;
    fs::create_dir_all(dir)?;

    let data = encode(data)?;
    fs::write(path, data)?;

    io::stdout().write_all(hash.as_bytes())?;
    Ok(())
}
