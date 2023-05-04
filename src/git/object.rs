mod blob;
mod commit;
mod kind;
mod tree;

use super::{
    codec::{Codable, Package},
    Sha,
};
use anyhow::{Context, Result};
pub use blob::Blob;
use bytes::{BufMut, BytesMut};
pub use commit::Commit;
pub use kind::Kind;
pub use tree::{Tree, TreeItem};

const GIT_BLOB_DELIMITER: u8 = b'\x00';
const GIT_KIND_DELIMITER: u8 = b' ';

//// HEADER
#[derive(Debug)]
pub struct Header {
    kind: Kind,
    size: usize,
}

impl Header {
    pub fn new(kind: Kind, size: usize) -> Self {
        Self { kind, size }
    }
}

impl Codable for Header {
    fn encode(&self, buffer: &mut BytesMut) -> Result<()> {
        self.kind.encode(buffer)?;
        buffer.put_u8(GIT_KIND_DELIMITER);
        let size = self.size.to_string();
        buffer.put(size.as_bytes());
        Ok(())
    }

    fn decode(chunk: &[u8]) -> Result<Self> {
        let mut parts = chunk.split(|b| *b == GIT_KIND_DELIMITER);
        let kind = parts.next().context("kind")?;
        let kind = Kind::decode(kind)?;
        let size = parts
            .next()
            .and_then(|s| std::str::from_utf8(s).ok())
            .and_then(|s| s.parse().ok())
            .context("size")?;
        Ok(Header { kind, size })
    }
}

/// Body
#[derive(Debug)]
pub enum Body {
    Blob(Blob),
    Tree(Tree),
    Commit(Commit),
}

impl Body {
    fn decode(kind: Kind, chunk: &[u8]) -> Result<Self> {
        Ok(match kind {
            Kind::Blob => Self::Blob(Blob::decode(chunk)?),
            Kind::Tree => Self::Tree(Tree::decode(chunk)?),
            Kind::Commit => Self::Commit(Commit::decode(chunk)?),
        })
    }

    fn encode(&self, buffer: &mut bytes::BytesMut) -> Result<()> {
        match self {
            Self::Blob(b) => b.encode(buffer),
            Self::Tree(t) => t.encode(buffer),
            Self::Commit(c) => c.encode(buffer),
        }
    }
}

// Object
#[derive(Debug)]
pub struct Object {
    header: Header,
    body: Body,
}

impl Object {
    pub fn new(header: Header, body: Body) -> Self {
        Self { header, body }
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn commit(tree: Sha, parent: Option<Sha>, message: String) -> Result<Self> {
        let commit = Commit {
            tree,
            parent,
            author: "Hernan Gonzalez".to_string(),
            committer: "Hernan Gonzalez".to_string(),
            message,
        };
        let body = Body::Commit(commit);
        let mut data = BytesMut::new();
        body.encode(&mut data)?;
        let header = Header {
            kind: Kind::Commit,
            size: data.len(),
        };
        Ok(Self { header, body })
    }
}

impl Codable for Object {
    fn decode(chunk: &[u8]) -> Result<Self> {
        let blob_ix = chunk
            .iter()
            .enumerate()
            .find(|t| t.1 == &GIT_BLOB_DELIMITER)
            .context("object header")?
            .0;
        let header = &chunk[..blob_ix];
        let body = &chunk[blob_ix + 1..];
        let header = Header::decode(header)?;
        let body = Body::decode(header.kind, body)?;
        Ok(Self { header, body })
    }

    fn encode(&self, buffer: &mut BytesMut) -> Result<()> {
        self.header.encode(buffer)?;
        buffer.put_u8(GIT_BLOB_DELIMITER);
        self.body.encode(buffer)
    }
}

impl Package for Object {}

pub struct ObjectBuilder<'a> {
    kind: Kind,
    data: &'a [u8],
}

impl<'a> ObjectBuilder<'a> {
    pub fn blob(data: &'a [u8]) -> Self {
        Self {
            kind: Kind::Blob,
            data,
        }
    }

    pub fn build(&self) -> Result<Object> {
        let header = Header {
            kind: self.kind,
            size: self.data.len(),
        };
        let body = match self.kind {
            Kind::Blob => Body::Blob(Blob::decode(self.data)?),
            Kind::Tree => Body::Tree(Tree::decode(self.data)?),
            Kind::Commit => Body::Commit(Commit::decode(self.data)?),
        };
        Ok(Object { header, body })
    }
}
