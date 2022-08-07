use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        Err("Usage: notes your_note_goes_here")?;
        std::process::exit(code: 1);
    }

    Ok(())
}