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

        sha: String,
    },
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
pub struct Cli {
    /// Name of the person to greet
    #[command(subcommand)]
    pub command: Command,
}
