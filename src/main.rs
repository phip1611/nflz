use std::io::stdin;
use nflz::{get_renamable_files, can_rename_all, NFLZError};
use std::process::exit;

fn main() {
    let pwd = std::env::current_dir().unwrap();
    let rn_map = get_renamable_files(&pwd).unwrap();
    if let Err(e) = can_rename_all(&pwd, &rn_map) {
        eprintln!("Can't rename the following files");
        match e {
            NFLZError::ConflictingFiles(files) => {
                println!("{:#?}", &files)
            }
            _ => {}
        }
        exit(-1)
    } else {

    }
}

/// Asks the user to confirm the action.
fn ask_for_confirmation() -> bool {
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
