mod pack;
mod refs;
mod transfer;

use anyhow::{ensure, Result};
use reqwest::Url;
use std::path::Path;
use transfer::GitTransfer;

pub struct Repository {
    url: Url,
}

impl Repository {
    pub fn new(url: Url) -> Self {
        Self { url }
    }

    pub async fn clone_at(self, dir: &Path) -> Result<()> {
        ensure!(dir.exists());
        let net = GitTransfer::new(self.url);
        let refs = net.refs().await?;
        let pack = net.pack_from(&refs).await?;

        dbg!(pack);

        todo!()
    }
}
