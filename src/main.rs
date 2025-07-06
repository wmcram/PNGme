mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use args::{Cli, PngMeArgs};
use clap::Parser;

fn main() -> Result<()> {
    let cli = crate::Cli::parse();

    match &cli.command {
        PngMeArgs::Encode(args) => {
            println!("Encode");
            Ok(())
        }
        PngMeArgs::Decode(args) => {
            println!("Decode");
            Ok(())
        }
        PngMeArgs::Remove(args) => {
            println!("Remove");
            Ok(())
        }
        PngMeArgs::Print(args) => {
            println!("Print");
            Ok(())
        }
    }
}
