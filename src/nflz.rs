use std::collections::HashMap;

#[derive(Debug)]
pub struct NumberIndices {
    pub start: u32,
    pub end: u32
}

impl NumberIndices {
    pub fn from(start: u32, end: u32) -> NumberIndices {
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
    number: u32,
    indices: NumberIndices
}

impl TransformationInformation {
    pub fn from(number: u32, indices: &NumberIndices) -> TransformationInformation {
        TransformationInformation {
            number,
            indices: NumberIndices::from_other(indices)
        }
    }
}

pub fn merge_maps<'a>(filename_number_map: HashMap<&'a String, u32>,
                  filename_number_indices_map: HashMap<&'a String, NumberIndices>)
    -> HashMap<&'a String, TransformationInformation> {
    let mut map = HashMap::new();

    for (k, v) in filename_number_map.iter() {
        map.insert(*k, TransformationInformation::from(*v, filename_number_indices_map.get(k).unwrap()));
    }

    let map = map;
    map
}

/*pub fn map_filenames(fn_indices_map: &HashMap<&String, NumberIndices) -> TransformationInformation {

}*/
