use clap::{Parser, Subcommand};
use std::path::PathBuf;

pub mod commands;

#[derive(Parser)]
#[command(name = "cync")]
#[command(about = "A command-line tool for the Cync programming language")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /* Run {
        #[arg(value_name = "FILE")]
        file: Option<PathBuf>,

        #[arg(short, long)]
        time: bool,
    },

    Build {
        #[arg(value_name = "FILE")]
        file: Option<PathBuf>,

        #[arg(short, long, value_name = "DIR")]
        output: Option<PathBuf>,

        #[arg(short, long)]
        verbose: bool,
    },*/
    Check {
        #[arg(value_name = "FILE")]
        file: Option<PathBuf>,

        #[arg(short, long)]
        verbose: bool,
    },
}
