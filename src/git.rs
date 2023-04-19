mod compress;
mod object;
mod sha;

use anyhow::{ensure, Context, Result};
use object::{AnyObject, GitObject};
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
    let obj = AnyObject::try_from(&sha)?;
    io::stdout().write_all(obj.bytes())?;
    Ok(())
}

pub fn hash_object(filename: &str) -> Result<()> {
    let path = Path::new(filename);
    ensure!(path.exists());

    let obj = AnyObject::try_from(path)?;
    let (sha, data) = obj.encode()?;

    let path = sha.path();
    let dir = path.parent().context("blob dir")?;
    fs::create_dir_all(dir)?;
    fs::write(path, data)?;

    io::stdout().write_all(sha.as_bytes())?;
    Ok(())
}

pub fn ls_tree(sha: &str, names: bool) -> Result<()> {
    ensure!(names, "Only names is supported");

    let sha: Sha = sha.try_into()?;
    let obj = AnyObject::try_from(&sha)?;
    let tree = obj.into_tree()?;
    for item in tree.items() {
        println!("{}", item.name);
    }

    Ok(())
}

pub fn write_tre() -> Result<()> {
    todo!()
}
