mod cli;
mod git;

use anyhow::Result;
use cli::*;

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Command::Init => git::init(),
        Command::CatFile { part } => git::cat_file(&part),
        Command::HashObject { file } => git::hash_object(&file),
        Command::LsTree { name_only, sha } => git::ls_tree(&sha, name_only),
    }
}
