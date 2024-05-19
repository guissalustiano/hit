#![feature(io_error_more)]

use log::debug;
use std::{
    fs::{self, File},
    io::ErrorKind,
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;

/// Create a file and parent directory if does not exist.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    path: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Cli::parse();
    let path = args.path;

    if path.exists() {
        debug!("path alredy exist");
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(&parent)?;
    }

    debug!("Try to create a path");
    match File::create_new(&path) {
        Err(err) if err.kind() == ErrorKind::IsADirectory => {
            debug!("It was a dir, try to create a dir");
            fs::create_dir(&path)?;
        }
        others => {
            others?;
        }
    };

    Ok(())
}
