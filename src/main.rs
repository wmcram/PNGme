mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use args::{Cli, PngMeArgs};
use clap::Parser;
use commands::*;

fn main() -> Result<()> {
    let cli = crate::Cli::parse();

    match cli.command {
        PngMeArgs::Encode(args) => encode(args),
        PngMeArgs::Decode(args) => decode(args),
        PngMeArgs::Remove(args) => remove(args),
        PngMeArgs::Print(args) => print_chunks(args),
        PngMeArgs::Get(args) => get_png(args),
    }
}
