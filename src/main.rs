#![cfg_attr(feature = "nightly", feature(panic_payload_as_str))]

use clap::{Parser, ValueEnum};
use crossterm::style::Stylize;
use log::{debug, error, info, LevelFilter};
use rustversion::nightly;
pub mod commands;
pub mod panic;
#[derive(Debug, ValueEnum, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
enum Distro {
    Arch,
    Debian,
}

impl std::fmt::Display for Distro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Distro::Arch => write!(f, "Arch Linux"),
            Distro::Debian => write!(f, "Debian"),
        }
    }
}
#[derive(Debug, Parser)]
#[clap(
    name = "Stellar",
    version = "0.1.0",
    about = "Stellar - A CLI/TUI for installing your favorite Linux distributions.",
    bin_name = "stellar"
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}
#[derive(Debug, clap::Subcommand)]
enum Commands {
    #[clap(alias = "i")]
    /// Install a Linux distribution
    Install {
        /// The distrobution you would like to install to install
        distro: Option<Distro>,
        #[clap(short, long)]
        tui: bool,
    },
    Debug,
}

fn match_distro(distro: Distro) {
    match distro {
        Distro::Arch => {
            info!("Installing Arch Linux");
            commands::install::arch::install();
        }
        Distro::Debian => {
            info!("Installing Debian");
        }
    }
}

fn main() {
    panic::set_panic_hook();

    rich_logger::init(LevelFilter::Debug).expect("Failed to init logger");
    let parser = Cli::try_parse();
    if rustversion::cfg!(nightly) {
        println!(
            "{}",
            "Beware! This version of Stellar was compiled with the nightly Rust compiler."
                .blue()
                .italic()
        );
    }
    match parser {
        Ok(cli) => match cli.command {
            Commands::Install { distro, tui } => {
                if tui {
                    error!("The TUI is not yet implemented");
                    todo!()
                }
                if let Some(distro) = distro {
                    match_distro(distro);
                } else {
                    debug!("No distro was selected, prompting user");
                    let distro: Distro = inquire::Select::new(
                        "Select the distro you would like to install",
                        vec![Distro::Arch, Distro::Debian],
                    )
                    .with_help_message(
                        "You can use the arrow keys to navigate and press Enter to select",
                    )
                    .prompt()
                    .expect("Failed to prompt");
                    match_distro(distro);
                }
            }
            Commands::Debug => panic!("Debugging"),
        },
        Err(err) => {
            err.print().expect("Failed to print error");
        }
    }
}
