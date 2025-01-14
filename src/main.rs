#![cfg_attr(feature = "nightly", feature(panic_payload_as_str))]
#![deny(unused_results)]
#![allow(missing_docs)]

pub mod cli;
pub mod panic;
use crate::cli::run_cli;
#[tokio::main]
async fn main() {	
	#[cfg(not(debug_assertions))]
	{
		panic::set_panic_hook();
	}
	run_cli().await;
}
