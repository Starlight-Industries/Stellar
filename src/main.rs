#![cfg_attr(feature = "nightly", feature(panic_payload_as_str))]
#![allow(unused_variables)]
use log::LevelFilter;
pub mod panic;
pub mod cli;
use cli::run_cli;

fn main() {
    #[cfg(not(debug_assertions))]
    panic::set_panic_hook();

    rich_logger::init(LevelFilter::Debug).expect("Failed to init logger");
    run_cli();
}
