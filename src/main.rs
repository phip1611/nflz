/*
MIT License

Copyright (c) 2021 Philipp Schuster

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
use std::io::stdin;
use std::path::{Path, PathBuf};
use std::process::exit;

fn main() {
    let dir = get_dir();

    if !dir.is_dir() {
        eprint!("NFLZ: {:?} is not a directory.", dir);
        exit(-1);
    } else {
        eprintln!("NFLZ: using dir: {:?}", dir);
    }

    let pf_list = nflz::get_matching_files(&dir).unwrap();
    let rn_map = nflz::compute_rename_map(&pf_list);
    if let Err(e) = nflz::can_rename_all(dir.as_path(), &rn_map, &pf_list) {
        eprintln!("Abort because at least one rename operation would result in a file that already exists:");
        match e {
            nflz::NFLZError::ConflictingFiles(files) => {
                println!("{:#?}", &files)
            }
            _ => {}
        }
        exit(-1)
    }

    println!("Would rename files:");
    let longest_old_name = rn_map
        .keys()
        .into_iter()
        .map(|k| k.original_filename().len())
        .max()
        .unwrap_or(0);
    for (old_file, new_name) in &rn_map {
        println!(
            "{}{} => {}",
            " ".repeat(longest_old_name - old_file.original_filename().len()),
            old_file,
            new_name
        );
    }

    let res = ask_for_confirmation();
    if !res {
        eprintln!("Exited");
        exit(0);
    }

    let _res = nflz::rename_all(&dir, &rn_map, &pf_list).unwrap();
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
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_s) => {
            // Strings equal?
            input.trim().to_lowercase() == "y" // trim to remove \r\n | \n
        }
        Err(_) => false,
    }
}
