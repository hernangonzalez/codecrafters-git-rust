mod codec;
mod object;
mod sha;

use self::codec::Package;
use anyhow::{ensure, Context, Result};
use object::{Blob, Body, Object, ObjectBuilder, Tree};
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
    let obj = Object::unpack(&sha)?;
    let Body::Blob(blob) = obj.body() else {
        return Err(anyhow::anyhow!("Not a blob"));
    };
    io::stdout().write_all(blob.as_bytes())?;
    Ok(())
}

pub fn hash_object(filename: &str) -> Result<()> {
    let chunk = fs::read(filename)?;
    let builder = ObjectBuilder::blob(&chunk);
    let obj = builder.build()?;
    let (sha, data) = obj.pack()?;

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
    let obj = Object::unpack(&sha)?;
    let Body::Tree(tree) = obj.body() else {
        return Err(anyhow::anyhow!("Not a tree"));
    };
    for item in tree.items() {
        println!("{}", item.name);
    }
    Ok(())
}

pub fn write_tree() -> Result<()> {
    let path = Path::new(".");
    let tree = Tree::read_path(path)?;
    dbg!(tree);
    todo!()
}
