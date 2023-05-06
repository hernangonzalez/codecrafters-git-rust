use anyhow::{ensure, Context, Result};
use bytes::{BufMut, Bytes, BytesMut};
use reqwest::{Client, Url};
use std::{path::Path, str::FromStr};

use super::Sha;

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
        dbg!(pack.len());
        todo!()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct GitRef {
    sha: Sha,
    name: String,
    size: usize,
}

impl FromStr for GitRef {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let (chunk, name) = s.split_once(' ').context("ref str")?;
        let (size, sha) = chunk.split_at(4);
        let sha = sha.parse::<Sha>()?;
        let name = name.to_string();
        let size = usize::from_str_radix(size, 16)?;
        Ok(Self { sha, name, size })
    }
}

struct GitTransfer {
    client: Client,
    remote: Url,
}

impl GitTransfer {
    fn new(remote: Url) -> Self {
        Self {
            client: Client::new(),
            remote,
        }
    }

    async fn refs(&self) -> Result<Vec<GitRef>> {
        let url = format!("{}/info/refs?service=git-upload-pack", self.remote);
        let req = self.client.get(url).build()?;
        let res = self.client.execute(req).await?;
        ensure!(res.status() == 200);
        let txt = res.text().await?;
        let vec = txt
            .lines()
            .filter_map(|s| s.parse::<GitRef>().ok())
            .collect();
        Ok(vec)
    }

    async fn pack_from(&self, refs: &[GitRef]) -> Result<Bytes> {
        let mut body = BytesMut::new();
        for r in refs {
            body.put_slice(b"0032want ");
            body.put_slice(r.sha.as_bytes());
            body.put_u8(b'\n');
        }
        body.put_slice(b"0000");

        let url = format!("{}/git-upload-pack", self.remote);
        let req = self.client.post(url).body(body.freeze()).build()?;
        let res = self.client.execute(req).await?;
        ensure!(res.status() == 200);
        let bytes = res.bytes().await?;
        Ok(bytes)
    }
}
