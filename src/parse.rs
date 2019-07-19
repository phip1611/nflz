use crate::globals::REGEX;
use std::collections::HashMap;
use regex::Regex;

pub fn get_number_indices(filenames: &Vec<String>) -> HashMap<&String, (u32, u32)> {
    let mut map = HashMap::new();
    for name in filenames {
        map.insert(name, get_number_indices_for_file(name));
    }
    let map = map;
    map
}

fn get_number_indices_for_file(filename: &str) -> (u32, u32) {
    let rx: Regex = REGEX.clone();
    let caps = rx.captures(filename).expect("Error applying regex to filename");
    let cap = caps.get(1).unwrap(); // 0 is the full match; 1 is what we want
    //println!("{:#?} - {}", cap.get(1), cap.len());
    (cap.start() as u32, cap.end() as u32)
}
