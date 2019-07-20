use crate::globals::REGEX;
use crate::math_util;
use std::collections::HashMap;
use regex::Regex;
use crate::nflz::TransformationInformation;

/// Gives back the Map with all information for the transformation (renaming) as well as the digit-count
/// of the biggest number
pub fn get_transformation_info(filenames: &Vec<String>) -> (HashMap<&String, TransformationInformation>, usize) {
    let mut map = HashMap::new();
    let mut max = 0; // to find the maximum number
    for name in filenames {
        // indices where (...)-group begins and ends
        let indices = get_number_indices_single(name);
        // number in (...)-group
        let number = get_number(name, &indices);
        if number > max {
            max = number;
        }
        map.insert(name, TransformationInformation::new(number, indices.0, indices.1));
    }
    let map = map;

    let digits = math_util::digits(max);

    (map, digits)
}

/// Returns the indices of the (...)-group for a specific filename-string.
fn get_number_indices_single(filename: &str) -> (usize, usize) {
    let rx: Regex = REGEX.clone();
    let caps = rx.captures(filename).expect("Error applying regex to filename");
    let cap = caps.get(1).unwrap(); // 0 is the full match; 1 is what we want
    //println!("{:#?} - {}", cap.get(1), cap.len());
    (cap.start(), cap.end())
}

/// Returns the number inside the (...)-group for a specific filename-string.
fn get_number(filename: &str, indices: &(usize, usize)) -> usize {
    let start = (indices.0 + 1) as usize; // skip '('
    let end   = (indices.1 - 1) as usize; // ignore ')'
    let s_num  = &filename[start .. end];
    s_num.parse::<usize>().unwrap()
}
