#![cfg_attr(feature = "nightly", feature(panic_payload_as_str))]
#![forbid(unused_results)]
#![cfg_attr(debug_assertions, allow(unused, dead_code))]
pub mod cli;
pub mod panic;
use crate::cli::run_cli;
use log::{debug, error, info, trace};

fn main() {
    let logger = rich_logger::init(log::LevelFilter::Info);

    match logger {
        Ok(_) => (),
        Err(e) => error!("Failed to initalize logger: {}", e),
    }
    #[cfg(not(debug_assertions))]
    {
        panic::set_panic_hook();
        if rustversion::cfg!(nightly) {
            info!("Hello, world!");
        }
    }
    trace!("Happy Halloween!");
    let distro = libstellar::env::current_distro();
    match distro {
        Ok(d) => info!("Distro: {}", d),
        Err(e) => error!("Failed to determine distro: {}", e),
    }
    run_cli();
}
