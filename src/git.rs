mod clone;
mod codec;
mod object;
mod sha;
mod tree_builder;

use self::{clone::Repository, codec::Package};
use anyhow::{ensure, Context, Result};
use object::{Body, Object, ObjectBuilder};
use reqwest::Url;
pub use sha::Sha;
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
    let sha: Sha = sha.parse()?;
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
    write_object(obj)
}

pub fn ls_tree(sha: &Sha, names: bool) -> Result<()> {
    ensure!(names, "Only names is supported");
    let obj = Object::unpack(sha)?;
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
    let skip = [DIR_GIT];
    let tree = tree_builder::tree_at_path(path, &skip)?;
    write_object(tree)
}

pub fn commit_tree(tree: Sha, commit: Sha, message: String) -> Result<()> {
    let obj = Object::commit(tree, Some(commit), message)?;
    write_object(obj)
}

pub async fn clone(url: Url, target: String) -> Result<()> {
    std::fs::create_dir_all(&target)?;
    let dir = Path::new(target.as_str());
    let repo = Repository::new(url);
    repo.clone_at(dir).await
}

fn write_object(obj: Object) -> Result<()> {
    let (sha, data) = obj.pack()?;
    let path = sha.path();
    let dir = path.parent().context("dir")?;
    fs::create_dir_all(dir)?;
    fs::write(path, data)?;
    io::stdout().write_all(sha.as_bytes())?;
    Ok(())
}
