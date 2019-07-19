use std::collections::HashMap;

#[derive(Debug)]
pub struct NumberIndices {
    pub start: usize,
    pub end: usize
}

impl NumberIndices {
    pub fn from(start: usize, end: usize) -> NumberIndices {
        NumberIndices {
            start,
            end
        }
    }

    pub fn from_other(x: &NumberIndices) -> NumberIndices {
        NumberIndices {
            start: x.start,
            end: x.end
        }
    }
}

#[derive(Debug)]
pub struct TransformationInformation {
    number: usize,
    indices: NumberIndices
}

impl TransformationInformation {
    pub fn from(number: usize, indices: &NumberIndices) -> TransformationInformation {
        TransformationInformation {
            number,
            indices: NumberIndices::from_other(indices)
        }
    }
}

pub fn merge_maps<'a>(filename_number_map: HashMap<&'a String, usize>,
                  filename_number_indices_map: HashMap<&'a String, NumberIndices>)
    -> HashMap<&'a String, TransformationInformation> {
    let mut map = HashMap::new();

    for (k, v) in filename_number_map.iter() {
        map.insert(*k, TransformationInformation::from(*v, filename_number_indices_map.get(k).unwrap()));
    }

    let map = map;
    map
}

pub fn get_new_filename_map<'a>(rename_map: &HashMap<&'a String, TransformationInformation>, digits: usize) -> HashMap<&'a String, String> {
    let mut map = HashMap::new();
    for (k, v) in rename_map.iter() {
        map.insert(*k, map_filename(k, v, digits));
    }
    let map = map;
    map
}

fn map_filename(name: &String, info: &TransformationInformation, digits: usize) -> String {
    let mut new_filename = String::from(&name[0 .. info.indices.start + 1]); // + 1 to include '('
    let digits_current = info.number / 10 + 1;
    for i in 0.. (digits - digits_current) {
        new_filename.push('0');
    }
    new_filename.push_str(&info.number.to_string());

    new_filename.push_str(&name[info.indices.end - 1 .. name.len()]); // - 1 to include ')'

    let new_filename = new_filename;
    new_filename
}