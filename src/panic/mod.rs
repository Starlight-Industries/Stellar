use std::{fmt::Write as FmtWrite, panic::PanicHookInfo};
use std::process::ExitCode;
use backtrace::Backtrace;
use crossterm::style::Stylize;
use log::info;
use regex::Regex;
static INIT: parking_lot::Once = parking_lot::Once::new();

pub fn set_panic_hook() {
    use std::io::Write;

    INIT.call_once(|| {
        let default_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let log_path = std::path::Path::new("./");
            let log_path = log_path.canonicalize().expect("Failed to obtain absolute path");
            if !log_path.exists() {
                std::fs::create_dir_all(&log_path).unwrap_or_else(|_| {
                    default_hook(info);
                    std::process::exit(1);
                })
            }
            let log_path = log_path.join("panic.log");

            // human_panic::print_msg::<PathBuf>(Some(log_path), &human_panic::Metadata::new("Stellar", env!("CARGO_PKG_VERSION"))
            // .support("https://github.com/Stellar-Engine/Stellar/issues")
            // .authors("Stellar community <https://github.com/Stellar-Engine>")).unwrap();
            // // Call the default hook for any additional actions

            let mut file = std::fs::File::create(&log_path).unwrap_or_else(|_| {
                default_hook(info);
                std::process::exit(1);
            });
            #[cfg(feature = "nightly")]
            writeln!(file, "{}", info.payload_as_str().unwrap_or_else(|| {
                default_hook(info);
                std::process::exit(0);
            })).unwrap_or_else(|_| {
                default_hook(info);
                std::process::exit(0);
            });
            writeln!(file, "{}", render_backtrace().sanitize_path()).unwrap_or_else(|_| {
                default_hook(info);
                std::process::exit(1);
            });
            let panic_str: Option<&&str> = info.payload().downcast_ref::<&str>();
            let panic_msg = format!(
                "Stellar had a problem and crashed. To help us diagnose the problem you can send us a crash report.

We have generated a report file at \"{}\". Submit an issue or email with the subject of \"Stellar Crash Report\" and include the report as an attachment.

To submit the crash report:

https://github.com/Starlight-industries/Stellar/issues

We take privacy seriously, and do not perform any automated error collection. In order to improve the software, we rely on people to submit reports.

Thank you kindly!",log_path.display());
            eprintln!("{}",panic_msg.red().bold());
            if let Some(panic_str) = panic_str {
                println!("{}\n{}",
                         "For future reference, the error message was as follows:".bold(),
                         panic_str.red().bold());
            }

            std::process::exit(1);
        }));
    });
}

// THIS SNIPPET IS LICENSED UNDER THE APACHE LICENSE, VERSION 2.0
// https://github.com/rust-cli/human-panic
// No changes were made to the original snippet
fn render_backtrace() -> String {
    //We take padding for address and extra two letters
    //to pad after index.
    #[allow(unused_qualifications)] // needed for pre-1.80 MSRV
    const HEX_WIDTH: usize = std::mem::size_of::<usize>() * 2 + 2;
    //Padding for next lines after frame's address
    const NEXT_SYMBOL_PADDING: usize = HEX_WIDTH + 6;

    let mut backtrace = String::new();

    //Here we iterate over backtrace frames
    //(each corresponds to function's stack)
    //We need to print its address
    //and symbol(e.g. function name),
    //if it is available
    let bt = Backtrace::new();
    let symbols = bt
        .frames()
        .iter()
        .flat_map(|frame| {
            let symbols = frame.symbols();
            if symbols.is_empty() {
                vec![(frame, None, "<unresolved>".to_owned())]
            } else {
                symbols
                    .iter()
                    .map(|s| {
                        (
                            frame,
                            Some(s),
                            s.name()
                                .map(|n| n.to_string())
                                .unwrap_or_else(|| "<unknown>".to_owned()),
                        )
                    })
                    .collect::<Vec<_>>()
            }
        })
        .collect::<Vec<_>>();
    let begin_unwind = "rust_begin_unwind";
    let begin_unwind_start = symbols
        .iter()
        .position(|(_, _, n)| n == begin_unwind)
        .unwrap_or(0);
    for (entry_idx, (frame, symbol, name)) in symbols.iter().skip(begin_unwind_start).enumerate() {
        let ip = frame.ip();
        let _ = writeln!(backtrace, "{entry_idx:4}: {ip:HEX_WIDTH$?} - {name}");
        if let Some(symbol) = symbol {
            //See if there is debug information with file name and line
            if let (Some(file), Some(line)) = (symbol.filename(), symbol.lineno()) {
                let _ = writeln!(
                    backtrace,
                    "{:3$}at {}:{}",
                    "",
                    file.display(),
                    line,
                    NEXT_SYMBOL_PADDING
                );
            }
        }
    }

    backtrace
}

trait Sanitize {
    fn sanitize_path(&self) -> String;
}

impl Sanitize for str {
    fn sanitize_path(&self) -> String {
        let username_pattern = r"(?i)(/home/|/Users/|\\Users\\)([^/\\]+)";
        let re = Regex::new(username_pattern).expect("Failed to sanitize path, aborting operation");

        re.replace_all(self, "${1}<USER>").to_string()
    }
}
