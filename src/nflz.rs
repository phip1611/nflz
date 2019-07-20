use crate::math_util;
use std::io::stdin;

/// Struct that describes the indices in the filename where the (...)-group was found
#[derive(Debug)]
struct TransformationIndicesInformation {
    pub start: usize,
    pub end: usize
}

impl TransformationIndicesInformation {
    /// Constructs a new TransformationIndicesInformation
    fn from(start: usize, end: usize) -> TransformationIndicesInformation {
        TransformationIndicesInformation {
            start,
            end
        }
    }
}

/// Struct that describes all information needed for the transformation/renaming.
#[derive(Debug)]
pub struct TransformationInformation {
    filename: String,
    pub number: usize,
    indices: TransformationIndicesInformation
}

impl TransformationInformation {
    /// Constructs a new TransformationInformation
    pub fn new(filename: &String, number: usize, indices: (usize, usize)) -> TransformationInformation {
        TransformationInformation {
            filename: String::from(filename),
            number,
            indices: TransformationIndicesInformation::from(
                indices.0,
                indices.1
            )
        }
    }
}

/// A Key-Value-Pair from old filename to new filename. We need this instead of a map
/// so that we can order it properly in a vector.
#[derive(Debug)]
pub struct RenameInformation {
    pub old_filename: String,
    pub new_filename: String
}

impl RenameInformation {
    /// Constructs a new RenameInformation
    pub fn new(old_filename: &String, new_filename: &String) -> RenameInformation {
        RenameInformation {
            old_filename: String::from(old_filename),
            new_filename: String::from(new_filename),
        }
    }
}

/// Returns the ordered vector with key value-pairs for the renaming.
pub fn get_new_filename_sorted_vector(rename_vec: &Vec<TransformationInformation>, max_digits: usize) -> Vec<RenameInformation> {
    let mut vec = Vec::new();
    let mut skipped_any = false;

    println!("nflz will skip the following files:");

    for ti in rename_vec.iter() {
        let new_filename = map_filename(ti, max_digits);

        // btw: need to compare the strings because we can't compare the number with max digits
        // because we have the number in TransformationInformation not in the string format
        // ==> zeros are stripped

        // Strings equals? (e.g. is nflz is running again in the same directory)
        if ti.filename == new_filename {
            skipped_any = true;
            println!("  {}", ti.filename); // print the skipped file
        } else {
            let ri = RenameInformation::new(&ti.filename, &new_filename);
            vec.push(ri);
        }
    }

    if skipped_any {
        println!(); // newline; then "nflz will rename the ..." is in next line
    } else {
        println!("  -\n"); // newline; then "nflz will rename the ..." is in next line
    }

    let vec = vec;
    vec
}

/// Transform the filename-string into the string with leading zeros in the (...)-group
fn map_filename(info: &TransformationInformation, digits: usize) -> String {
    let mut new_filename = String::from(&info.filename[0 .. info.indices.start + 1]); // + 1 to include '('
    let digits_current = math_util::digits(&info.number);
    for _i in 0 .. (digits - digits_current) {
        new_filename.push('0');
    }
    new_filename.push_str(&info.number.to_string());

    new_filename.push_str(&info.filename[info.indices.end - 1 .. info.filename.len()]); // - 1 to include ')'

    let new_filename = new_filename;
    new_filename
}

/// Shows the user all files that are going to be renamed
pub fn print_intended_actions(vec: &Vec<RenameInformation>) {
    if vec.len() == 0 {
        // this will happen if there are files in the directory but they are already
        // ALL renamed
        println!("No files to rename left; will exit");
        return;
    } else {
        println!("nflz will rename the following files:");
    }

    let mut longest_name_len = 0;

    // we need the longest name so that we can add spaces
    // to the key beeing printed; so we get something like this
    // stdout:
    //  paris (2).txt   => paris (2).txt
    //  paris (1).txt => paris (1).txt
    // rather than
    //  paris (2).txt => paris (2).txt
    //  paris (1).txt => paris (1).txt

    // Loop to find the longest (...)-group
    for ri in vec.iter() {
        let l = ri.old_filename.len();
        if l > longest_name_len {
            longest_name_len = l;
        }
    }

    // Printing all key-value-pairs from old filename to new filename
    for ri in vec.iter() {
        let mut x = String::from(&ri.old_filename);
        let range = longest_name_len - ri.old_filename.len();
        for _i in 0 .. range {
            x.push(' ');
        }
        println!("  {} => {}", x, ri.new_filename);
    }
}

/// Asks the user to confirm the action.
pub fn ask_for_confirmation() -> bool {
    println!("\nPlease confirm with 'y' or abort with 'n'");
    let mut input= String::new();
    match stdin().read_line(&mut input) {
        Ok(_s) => {
            // Strings equal?
            input.trim() == String::from("y") // trim to remove \r\n | \n
        }
        Err(_) => false
    }
}
