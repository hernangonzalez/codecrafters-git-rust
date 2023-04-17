mod compress;
mod object;
mod sha;

use anyhow::{ensure, Context, Result};
use object::Object;
use sha::Sha;
use std::{
    fs,
    io::{self, Write},
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
    let sha: Sha = sha.try_into()?;
    let data = fs::read(sha.path())?;
    let data = compress::decode(&data)?;

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

pub fn hash_object(filename: &str) -> Result<()> {
    let path = Path::new(filename);
    ensure!(path.exists());

    let blob = fs::read(path)?;
    let obj = Object::blob(blob);
    let (sha, data) = obj.encode()?;

    let path = sha.path();
    let dir = path.parent().context("blob dir")?;
    fs::create_dir_all(dir)?;
    fs::write(path, data)?;

    io::stdout().write_all(sha.as_bytes())?;
    Ok(())
}
