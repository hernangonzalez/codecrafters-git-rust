use crate::git::{codec::Codable, Sha};
use anyhow::Result;
use bytes::{BufMut, BytesMut};

const GIT_COMMIT_NEWLINE: u8 = b'\n';

#[derive(Debug)]
pub struct Commit {
    pub tree: Sha,
    pub parent: Option<Sha>,
    pub author: String,
    pub committer: String,
    pub message: String,
}

impl Codable for Commit {
    fn encode(&self, buffer: &mut BytesMut) -> Result<()> {
        buffer.put_slice(b"tree ");
        buffer.put_slice(self.tree.as_bytes());
        buffer.put_u8(GIT_COMMIT_NEWLINE);

        if let Some(parent) = &self.parent {
            buffer.put_slice(b"parent ");
            buffer.put_slice(parent.as_bytes());
        }
        buffer.put_u8(GIT_COMMIT_NEWLINE);

        let author = format!("author {} {}", self.author, "1493170892 -0500");
        buffer.put_slice(author.as_bytes());
        buffer.put_u8(GIT_COMMIT_NEWLINE);

        let commiter = format!("committer {} {}", self.committer, "1493170892 -0500");
        buffer.put_slice(commiter.as_bytes());
        buffer.put_u8(GIT_COMMIT_NEWLINE);

        buffer.put_u8(GIT_COMMIT_NEWLINE);
        buffer.put_slice(self.message.as_bytes());
        buffer.put_u8(GIT_COMMIT_NEWLINE);

        Ok(())
    }

    fn decode(_chunk: &[u8]) -> Result<Self> {
        todo!()
    }
}
