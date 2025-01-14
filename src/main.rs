#![cfg_attr(feature = "nightly", feature(panic_payload_as_str))]
#![deny(unused_results)]
#![allow(missing_docs)]

pub mod cli;
pub mod panic;
use log::debug;

use crate::cli::run_cli;
#[tokio::main]
async fn main() {
	let current_user = sudo::check();
	panic::set_panic_hook();
	match current_user {
		sudo::RunningAs::Root => {
			debug!("running as root")
		},
		sudo::RunningAs::User => {
			eprintln!("You must run stellar as either sudo or root!");
			let should_continue = inquire::Confirm::new("Would you like to rerun as sudo?")
				.prompt().expect("Failed to prompt user");
			if !should_continue {
				std::process::exit(1)
			}
			let _ = sudo::escalate_if_needed();
		},
		sudo::RunningAs::Suid => {
			debug!("Running as sudo")
		},
	}
	#[cfg(not(debug_assertions))]
	{


	}
	run_cli().await;
}
