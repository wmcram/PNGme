use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: PngMeArgs,
}

#[derive(Subcommand)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Args)]
pub struct EncodeArgs {
    filename: PathBuf,
    chunk_type: String,
    message: String,
    output_filename: Option<PathBuf>,
}
#[derive(Args)]
pub struct DecodeArgs {
    filename: PathBuf,
    chunk_type: String,
}
#[derive(Args)]
pub struct RemoveArgs {
    filename: PathBuf,
    chunk_type: String,
}
#[derive(Args)]
pub struct PrintArgs {
    filename: PathBuf,
}
