use super::{Sha, GIT_BLOB_DELIMITER, GIT_KIND_DELIMITER};

#[derive(Debug)]
pub struct Tree(Vec<u8>);

#[derive(Debug)]
pub struct TreeItem<'a> {
    pub mode: &'a str,
    pub name: &'a str,
    pub sha: Sha,
}

impl<'a> Tree {
    pub fn new(inner: Vec<u8>) -> Self {
        Self(inner)
    }

    pub fn items(&'a self) -> Vec<TreeItem<'a>> {
        let scanner = TreeScanner { data: &self.0 };
        scanner.collect()
    }
}

struct TreeScanner<'a> {
    data: &'a [u8],
}

impl<'a> Iterator for TreeScanner<'a> {
    type Item = TreeItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let data = self.data;
        let mode_at = data.iter().position(|b| *b == GIT_KIND_DELIMITER)?;
        let (mode, data) = data.split_at(mode_at);
        let mode = std::str::from_utf8(mode).ok()?;

        let data = &data[1..];
        let name_at = data.iter().position(|b| *b == GIT_BLOB_DELIMITER)?;
        let (name, data) = data.split_at(name_at);
        let name = std::str::from_utf8(name).ok()?;
        let data = &data[1..];

        let (hex, data) = data.split_at(20);
        let hex = hex::encode(hex);
        let sha: Sha = hex.as_str().try_into().ok()?;

        let item = TreeItem { mode, name, sha };
        self.data = data;
        Some(item)
    }
}
