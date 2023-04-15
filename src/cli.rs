pub use clap::Parser;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Command {
    Init,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
pub struct Cli {
    /// Name of the person to greet
    #[command(subcommand)]
    pub command: Command,
}
