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
        eprintln!("No matching files found in this directory.");
        return;
    }

    //eprintln!("{:#?}", filenames);

    // filename Map
    let filename_number_indices_map = parse::get_number_indices(&filenames);
    //eprintln!("{:#?}", filename_number_indices_map);

    let filename_number_map = parse::get_numbers(&filename_number_indices_map);

    let max = filename_number_map.values().max().unwrap(); // finding the max number
    let max_digits: usize = math_util::digits(*max);

    // Map with all information that we need for the transformation
    let final_transform_map = nflz::merge_maps(
        filename_number_map,
        filename_number_indices_map
    );

    // TODO: instead of making two maps and merging them make the one map at once

    println!("{:#?}", final_transform_map);


    let rename_map = nflz::get_new_filename_map(&final_transform_map, max_digits);

    println!("{:#?}", rename_map);

    nflz::rename_all_files(rename_map);
}

