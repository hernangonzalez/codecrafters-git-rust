use crate::git::Sha;
pub use clap::Parser;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Command {
    Init,

    CatFile {
        #[arg(short = 'p')]
        part: String,
    },

    HashObject {
        #[arg(short = 'w')]
        file: String,
    },

    LsTree {
        #[arg(long = "name-only")]
        name_only: bool,

        sha: Sha,
    },

    WriteTree,

    CommitTree {
        sha: Sha,

        #[arg(short = 'p')]
        commit_sha: Sha,

        #[arg(short = 'm')]
        message: String,
    },
}

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}
