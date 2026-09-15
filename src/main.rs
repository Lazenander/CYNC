extern crate core;
mod cli;
mod compiler;
pub mod pipeline;
mod runtime;
pub mod utility;

use clap::Parser;
use cli::{commands, Cli, Commands};
use std::{env, process};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Check { file, verbose } => commands::check::execute(
            file.unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| ".".into())),
        ),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
