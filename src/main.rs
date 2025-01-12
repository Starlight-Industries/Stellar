#![cfg_attr(feature = "nightly", feature(panic_payload_as_str))]
#![forbid(unused_results)]
#![cfg_attr(debug_assertions, allow(unused, dead_code))]
pub mod cli;
pub mod panic;
use crate::cli::run_cli;
use log::{debug, error, info, trace};

fn main() {
    #[cfg(not(debug_assertions))]
    {
        panic::set_panic_hook();
    }
    run_cli();
}
