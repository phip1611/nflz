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

    eprintln!("{:#?}", filenames);

    // filename Map
    let number_indices = parse::get_number_indices(&filenames);
    eprintln!("{:#?}", number_indices);
    /*
    let max_digits = get_max_digits(&filenames);

    let fn_map: HashMap<&str, String> = HashMap::new();*/
}

