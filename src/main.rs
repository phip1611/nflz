#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;

mod validation;
mod parse;
mod fs_util;
mod globals;
mod nflz;
mod math_util;

fn main() {
    let pwd= env::current_dir()
        .expect("Can't get current working directory!")
        .display().to_string();
    let filenames = fs_util::get_files(pwd);

    if filenames.len() == 0 {
        println!("No matching files found in this directory.");
        return;
    }

    if filenames.len() == 1 {
        println!("There is only one file found. No action needed.");
        return;
    }

    // map with all infos we need for the renaming
    let (vec, max_digits) = parse::get_transformation_info(&filenames);

    let rename_vec = nflz::get_new_filename_sorted_vector(&vec, max_digits);

    nflz::print_intended_actions(&rename_vec);

    if rename_vec.len() == 0 {
        return;
    }

    let confirmed = nflz::ask_for_confirmation();

    if confirmed {
        fs_util::rename_all_files(&rename_vec);
        println!("\nDone.");
    } else {
        println!("Aborted.");
    }

}

