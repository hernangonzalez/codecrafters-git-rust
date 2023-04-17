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
    let obj = Object::decode(data)?;
    io::stdout().write_all(&obj.data)?;
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
