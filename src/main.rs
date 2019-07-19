#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;

mod validation;
mod parse;
mod fs_util;
mod globals;

fn main() {
    let pwd= env::current_dir()
        .expect("Can't get current working directory!")
        .display().to_string();
    let filenames = fs_util::get_files(pwd);

    //eprintln!("{:#?}", filenames);

    // filename Map
    let number_indices = parse::get_number_indices(&filenames);
    //eprintln!("{:#?}", number_indices);

    let numbers: Vec<u32> = parse::get_numbers(&number_indices);
    let max = numbers.iter().max().unwrap(); // finding the max number
    let digits: u32 = max / 10 + 1;

    //eprintln!("{:#?} - max: {} with {} digits", numbers, max, digits);
    /*
    let max_digits = get_max_digits(&filenames);

    let fn_map: HashMap<&str, String> = HashMap::new();*/
}

