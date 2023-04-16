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
    }
}
