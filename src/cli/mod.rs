use clap::{Error, Parser, ValueEnum};
use clap_verbosity_flag::{Verbosity, VerbosityFilter};
use crossterm::style::Stylize;
use libstellar::env::Distro;
use log::{debug, error, info, LevelFilter};
// pub mod commands;
// pub mod install;

#[derive(Debug, Parser)]
#[clap(
    name = "Stellar",
    version = "0.1.0",
    about = "Stellar - A CLI/TUI for installing your favorite Linux distributions.",
    bin_name = "stellar"
)]
struct Cli {
    #[command(flatten)]
    verbose: Verbosity,
    #[clap(subcommand)]
    command: Commands,
}
#[derive(Debug, clap::Subcommand)]
enum Commands {
    #[clap(alias = "i")]
    /// Install a Linux distribution
    Install {
        /// The distribution you would like to install to install
        distro: Option<String>,
        #[clap(short, long)]
        tui: bool,
    },
    Debug,
}
fn match_distro(distro: Distro) {
    if !distro.is_supported() {
        error!("Distro not supported");
        return;
    }
    match distro {
        Distro::Debian => {
            info!("Debian");
        }
        Distro::Arch => {
            info!("Arch");
        }
        _ => {
            unreachable!();
            // error!("Distro not supported");
        }
    }
}

pub fn run_cli() {
    let parser = Cli::try_parse();

    #[cfg(not(debug_assertions))]
    if rustversion::cfg!(nightly) {
        println!(
            "{}",
            "Beware! This version of Stellar was compiled with the nightly Rust compiler."
                .blue()
                .italic()
        );
    }

    match parser {
        Ok(cli) => {
            let verbosity = cli.verbose.filter();
            let filter = LevelFilter::from(verbosity);
            let logger = rich_logger::init(filter);

            match logger {
                Ok(_) => (),
                Err(e) => error!("Failed to initalize logger: {}", e),
            }
            match cli.command {
                Commands::Install { distro, tui } => {
                    if tui {
                        error!("The TUI is not yet implemented");
                        todo!()
                    }
                    if let Some(distro) = distro {
                        match distro.parse::<Distro>() {
                            Ok(d) => {
                                match_distro(d);
                            }
                            Err(e) => {
                                error!("Failed to parse distro: {}", e);
                            },
                        }
                    } else {
                        debug!("No distro was selected, prompting user");
                        let distro: Distro = inquire::Select::new(
                            "Select the distro you would like to install",
                            Distro::All(),
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
            }
        }
        Err(err) => {
            err.print().expect("Failed to print error");
        }
    }
}
