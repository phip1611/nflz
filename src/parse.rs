use crate::globals::REGEX;
use std::collections::HashMap;
use regex::Regex;
use crate::nflz::NumberIndices;

pub fn get_number_indices(filenames: &Vec<String>) -> HashMap<&String, NumberIndices> {
    let mut map = HashMap::new();
    for name in filenames {
        map.insert(name, get_number_indices_single(name));
    }
    let map = map;
    map
}

fn get_number_indices_single(filename: &str) -> NumberIndices {
    let rx: Regex = REGEX.clone();
    let caps = rx.captures(filename).expect("Error applying regex to filename");
    let cap = caps.get(1).unwrap(); // 0 is the full match; 1 is what we want
    //println!("{:#?} - {}", cap.get(1), cap.len());
    NumberIndices::from(cap.start() as usize, cap.end() as usize)
}

// 'a: lifetime parameter, see https://users.rust-lang.org/t/returning-values-borrowed-from-multiple-places/1350
pub fn get_numbers<'a>(filenames: &HashMap<&'a String, NumberIndices>) -> HashMap<&'a String, usize> {
    let mut map= HashMap::new();
    for (k, v) in filenames.iter() {
        map.insert(*k, get_number(k, v));
    }
    let map = map;
    map
}

fn get_number(filename: &str, indices: &NumberIndices) -> usize {
    let start = (indices.start + 1) as usize; // skip '('
    let end   = (indices.end - 1) as usize; // ignore ')'
    let s_num  = &filename[start .. end];
    s_num.parse::<usize>().unwrap()
}
