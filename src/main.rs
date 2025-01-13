#![cfg_attr(feature = "nightly", feature(panic_payload_as_str))]
#![forbid(unused_results)]

pub mod cli;
pub mod panic;
use crate::cli::run_cli;

fn main() {
    #[cfg(not(debug_assertions))]
    {
        panic::set_panic_hook();
    }
    run_cli();
}
