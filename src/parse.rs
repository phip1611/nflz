use crate::globals::REGEX;
use crate::math_util;
use regex::Regex;
use crate::nflz::TransformationInformation;

/// Gives back the sorted Vector with all information for the transformation (renaming) as well as the digit-count
/// of the biggest number
pub fn get_transformation_info(filenames: &Vec<String>) -> (Vec<TransformationInformation>, usize) {
    let mut vec = Vec::new();
    let mut max = 0; // to find the maximum number

    for name in filenames {
        // indices where (...)-group begins and ends
        let indices = get_number_indices_single(name);
        // number in (...)-group
        let number = get_number(name, &indices);
        if number > max {
            max = number;
        }

        let ti = TransformationInformation::new(name, number, indices);
        vec.push(ti);
    }

    // sort all filenames by their number in natural order // when we print the files they have a easy to read order (instead of random order)
    vec.sort_by(|a, b| a.number.cmp(&b.number));

    let digits = math_util::digits(&max);
    let vec = vec;
    (vec, digits)
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
