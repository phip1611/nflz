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
    let (map, max_digits) = parse::get_transformation_info(&filenames);

    let rename_map = nflz::get_new_filename_map(&map, max_digits);

    nflz::show_user_intended_actions(&rename_map);

    nflz::rename_all_files(rename_map);
}

