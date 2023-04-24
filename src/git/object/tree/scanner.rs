use super::{Sha, TreeItem, GIT_BLOB_DELIMITER, GIT_KIND_DELIMITER};

pub struct TreeScanner<'a> {
    pub data: &'a [u8],
}

impl<'a> Iterator for TreeScanner<'a> {
    type Item = TreeItem;

    fn next(&mut self) -> Option<Self::Item> {
        let data = self.data;
        let mode_at = data.iter().position(|b| *b == GIT_KIND_DELIMITER)?;
        let (mode, data) = data.split_at(mode_at);
        let mode = std::str::from_utf8(mode).ok()?.to_string();

        let data = &data[1..];
        let name_at = data.iter().position(|b| *b == GIT_BLOB_DELIMITER)?;
        let (name, data) = data.split_at(name_at);
        let name = std::str::from_utf8(name).ok()?.to_string();
        let data = &data[1..];

        let (hex, data) = data.split_at(20);
        let hex = hex::encode(hex);
        let sha: Sha = hex.as_str().try_into().ok()?;

        let item = TreeItem { mode, name, sha };
        self.data = data;
        Some(item)
    }
}
