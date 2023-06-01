use super::Sha;
use anyhow::{ensure, Context, Result};
use bytes::{BufMut, Bytes, BytesMut};
use reqwest::{header, Client, Url};
use std::{path::Path, str::FromStr};

const GIT_PACK_WANT: &[u8] = b"0032want ";
const GIT_PACK_NEWLINE: u8 = b'\n';
const GIT_PACK_END: &[u8] = b"0000";
const GIT_PACK_DONE: &[u8] = b"0009done";
const HTTP_CONTENT_TYPE: &str = "application/x-www-form-urlencoded";

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

    /// ref https://man.archlinux.org/man/gitprotocol-pack.5.en
    async fn pack_from(&self, refs: &[GitRef]) -> Result<Bytes> {
        let url = format!("{}/git-upload-pack", self.remote);

        let mut body = BytesMut::new();
        for r in refs {
            body.put_slice(GIT_PACK_WANT);
            body.put_slice(r.sha.as_bytes());
            body.put_u8(GIT_PACK_NEWLINE);
        }
        body.put_slice(GIT_PACK_END);
        body.put_slice(GIT_PACK_DONE);
        body.put_u8(GIT_PACK_NEWLINE);
        let body = body.freeze();

        let req = self
            .client
            .post(url)
            .header(header::CONTENT_TYPE, HTTP_CONTENT_TYPE)
            .body(body)
            .build()?;

        let res = self.client.execute(req).await?;
        ensure!(res.status() == 200);
        let bytes = res.bytes().await?;
        Ok(bytes)
    }
}
