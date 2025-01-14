use std::fs;

use clap::Parser;
use clap_verbosity_flag::Verbosity;
use libstellar::{bootstrap::arch::Pacstrap, chroot::Chroot};
use libstellar::bootstrap::Bootstrap;
use libstellar::env::Distro;
use log::{debug, error, info, LevelFilter};
// pub mod commands;

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
async fn match_distro(distro: Distro) {
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
			// let mut bootstrapper = Pacstrap::default();
			// bootstrapper.add_package("base").await.unwrap();
			// bootstrapper.add_package("base-devel").await.unwrap();
			// bootstrapper.add_package("pipewire").await.unwrap();
			// bootstrapper.add_package("plasma-meta").await.unwrap();
			// let result = fs::create_dir_all("./test/chroot");
			// match result {
			// 	Ok(_) => {
			// 		bootstrapper.bootstrap("./test/chroot").unwrap();
			// 	}
			// 	Err(e) => {
			// 		error!("{}",e);
			// 		todo!()
			// 	}
			// }
			let test = Chroot::new("t");

		}
		Distro::Gentoo => {
			info!("Gentoo")
		}
		_ => {
			unreachable!();
			// error!("Distro not supported");
		}
	}
}

pub async fn run_cli() {
	let parser = Cli::try_parse();

	#[cfg(not(debug_assertions))]
	if rustversion::cfg!(nightly) {
		use crossterm::style::Stylize;
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
								match_distro(d).await;
							}
							Err(e) => {
								error!("Failed to parse distro: {}", e);
							}
						}
					} else {
						debug!("No distro was selected, prompting user");
						let distro: Distro = inquire::Select::new(
							"Select the distro you would like to install",
							Distro::variants(),
						)
						.with_help_message(
							"You can use the arrow keys to navigate and press Enter to select",
						)
						.prompt()
						.expect("Failed to prompt");
						match_distro(distro).await;
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
