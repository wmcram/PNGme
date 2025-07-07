use std::fs;
use std::str::FromStr;

use crate::Result;
use crate::args::*;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let mut png = Png::from_file(args.filename.clone())?;
    png.append_chunk(Chunk::new(
        ChunkType::from_str(&args.chunk_type)?,
        args.message.into_bytes(),
    ));
    let out = args.output_filename.unwrap_or(args.filename);
    let bytes = png.as_bytes();
    fs::write(out, bytes)?;
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let png = Png::from_file(args.filename)?;
    if let Some(chunk) = png.chunk_by_type(&args.chunk_type) {
        println!("{}", chunk.data_as_string()?);
    }
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = Png::from_file(args.filename.clone())?;
    match png.remove_first_chunk(&args.chunk_type) {
        Ok(chunk) => {
            fs::write(&args.filename, png.as_bytes())?;
            println!("Removed chunk {chunk}")
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let png = Png::from_file(args.filename)?;
    for chunk in png.chunks() {
        println!("{chunk}");
    }
    Ok(())
}

// Downloads a png from the internet
pub fn get_png(args: GetArgs) -> Result<()> {
    let png = Png::from_url(&args.url)?;
    fs::write(&args.filename, png.as_bytes())?;
    Ok(())
}
