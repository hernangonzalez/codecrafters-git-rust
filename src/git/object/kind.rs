use anyhow::Result;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Kind {
    Blob,
    Tree,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blob => write!(f, "blob"),
            Self::Tree => write!(f, "tree"),
        }
    }
}

impl TryFrom<&str> for Kind {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self> {
        match value {
            "blob" => Ok(Self::Blob),
            "tree" => Ok(Self::Tree),
            k => Err(anyhow::anyhow!("Unknown kind: {k}")),
        }
    }
}
