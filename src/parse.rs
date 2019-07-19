use crate::globals::REGEX;
use std::collections::HashMap;
use regex::Regex;
use std::borrow::Borrow;
use std::ops::Deref;

pub fn get_number_indices(filenames: &Vec<String>) -> HashMap<&String, (u32, u32)> {
    let mut map = HashMap::new();
    for name in filenames {
        map.insert(name, get_number_indices_single(name));
    }
    let map = map;
    map
}

fn get_number_indices_single(filename: &str) -> (u32, u32) {
    let rx: Regex = REGEX.clone();
    let caps = rx.captures(filename).expect("Error applying regex to filename");
    let cap = caps.get(1).unwrap(); // 0 is the full match; 1 is what we want
    //println!("{:#?} - {}", cap.get(1), cap.len());
    (cap.start() as u32, cap.end() as u32)
}

pub fn get_numbers(filenames: &HashMap<&String, (u32, u32)>) -> Vec<u32> {
    let mut vec= Vec::new();
    for (k, v) in filenames.iter() {
        vec.push(get_number(k, v));
    }
    let vec = vec;
    vec
}

fn get_number(filename: &str, indices: &(u32, u32)) -> u32 {
    let start = (indices.0 + 1) as usize; // skip '('
    let end   = (indices.1 - 1) as usize; // ignore ')'
    let s_num  = &filename[start .. end];
    s_num.parse::<u32>().unwrap()
}
