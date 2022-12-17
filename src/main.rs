#![allow(unused)]

use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, BufRead};
use std::path::Path;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let lines = read_lines(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path.display()))?;

    for line in lines {
        let ip: String = line?;
        if ip.contains(&args.pattern) {
            println!("{}", ip);
        }
    }

    Ok(())
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
