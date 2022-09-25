/*
MIT License

Copyright (c) 2022 Philipp Schuster

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

#![deny(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    // clippy::restriction,
    // clippy::pedantic
)]
// now allow a few rules which are denied by the above statement
// --> they are ridiculous and not necessary
#![allow(
    clippy::suboptimal_flops,
    clippy::redundant_pub_crate,
    clippy::fallible_impl_from
)]
// only for lib, not for bin
// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(rustdoc::all)]

use log::LevelFilter;
use nflz::{NFLZAssistant, NFLZError};
use std::io::stdin;
use std::path::{Path, PathBuf};
use std::process::exit;

fn main() {
    let dir = get_dir();

    log::set_logger(&logger::StdErrLogger).unwrap();
    log::set_max_level(LevelFilter::max());

    let assistant = NFLZAssistant::new(dir);

    if let Err(err) = assistant {
        println!(
            "Can't perform the desired action on the given directory. Error:\n{}",
            err
        );
        exit(1);
    }
    let assistant = assistant.unwrap();

    if assistant.files_to_rename().is_empty() {
        println!("Found no files to rename. Exit.");
        exit(0);
    }

    println!("NFLZ would not rename the following files:");
    for skipped_file in assistant.files_without_rename() {
        println!("  {}", skipped_file.file_info().original_filename());
    }

    println!("NFLZ would rename the following files:");
    for file in assistant.files_to_rename() {
        // todo make this more dynamic
        println!(
            "  {:25} => {}",
            file.file_info().original_filename(),
            file.new_filename().expect("must exist at that point"),
        );
    }

    let res = ask_for_confirmation();
    if !res {
        println!("Aborted");
        exit(0);
    }

    let res = assistant.rename_all();

    match res {
        Ok(files) => {
            let renamed_files_count = files
                .iter()
                .filter(|x| !x.is_already_properly_named())
                .count();
            let unchanged_files_count = files
                .iter()
                .filter(|x| x.is_already_properly_named())
                .count();
            println!(
                "Successfully renamed {} files. {} files did not need to be renamed.",
                renamed_files_count, unchanged_files_count
            );
        }
        Err(err) => match &err {
            NFLZError::AmbiguousPrefixes(_) | NFLZError::AmbiguousSuffixes(_) => {
                println!(
                    "Aborted renaming early. No changes made to the file system. Error is:\n{}",
                    err
                );
            }
            NFLZError::RenameFailed(old, new, ioerror) => {
                println!("Failure during renaming. File state might be inconsistent now.");
                println!(
                    "Could not rename '{}' to '{} because of: {}'",
                    old, new, ioerror
                );
            }
            _ => {
                panic!("Unexpected error! {:#?}", err);
            }
        },
    }
}

/// Returns either PWD or the dir specified by first argument as [`PathBuf`].
fn get_dir() -> PathBuf {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        Path::new(&args[1]).to_path_buf()
    } else {
        std::env::current_dir().unwrap()
    }
}

/// Asks the user to confirm the action.
fn ask_for_confirmation() -> bool {
    println!("\nPlease confirm with 'y' or abort with 'n'");
    println!("  NFLZ can't guarantee you 100% safety. Always make a backup first (:");
    println!(
        "  But to the best of my knowledge this should work if no catastrophic failure occurs."
    );
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_s) => {
            // Strings equal?
            input.trim().to_lowercase() == "y" // trim to remove \r\n | \n
        }
        Err(_) => false,
    }
}

mod logger {
    use log::{Metadata, Record};

    pub struct StdErrLogger;

    impl log::Log for StdErrLogger {
        fn enabled(&self, _metadata: &Metadata) -> bool {
            true
        }

        fn log(&self, record: &Record) {
            eprintln!(
                "[{:5}] @ {}:{}: {}",
                record.level(),
                record.file().unwrap_or("<unknown>"),
                record.line().unwrap_or(0),
                record.args()
            );
        }

        fn flush(&self) {}
    }
}
