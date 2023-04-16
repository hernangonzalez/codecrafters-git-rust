use anyhow::{ensure, Context, Result};
use flate2::read::ZlibDecoder;
use std::{
    fs::{self, File},
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
    ensure!(sha.len() == 40, "Unexpected sha1 hash");
    let (prefix, file_name) = sha.split_at(2);

    let path = Path::new(DIR_GIT_OBJECTS).join(prefix).join(file_name);
    ensure!(path.exists(), "Blob does not exists");

    let mut file = File::open(path)?;
    let mut data = vec![];
    file.read_to_end(&mut data)?;

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
