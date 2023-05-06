mod cli;
mod git;

use anyhow::Result;
use cli::*;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Command::Init => git::init(),
        Command::CatFile { part } => git::cat_file(&part),
        Command::HashObject { file } => git::hash_object(&file),
        Command::LsTree { name_only, sha } => git::ls_tree(&sha, name_only),
        Command::WriteTree => git::write_tree(),
        Command::CommitTree {
            sha,
            commit_sha,
            message,
        } => git::commit_tree(sha, commit_sha, message),
        Command::Clone { url, target } => git::clone(url, target).await,
    }
}
