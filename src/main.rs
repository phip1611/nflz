use std::io::stdin;
use nflz::{get_renamable_files, can_rename_all, NFLZError};
use std::process::exit;
use std::path::{Path, PathBuf};

fn main() {
    let dir = get_dir();

    if !dir.is_dir() {
        eprint!("NFLZ: {:?} is not a directory.", dir);
        exit(-1);
    } else {
        eprintln!("NFLZ: using dir: {:?}", dir);
    }

    let rn_map = get_renamable_files(dir.as_path()).unwrap();
    if let Err(e) = can_rename_all(dir.as_path(), &rn_map) {
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

/// Returns either PWD or the dir specified by first argument as [`PathBuf`].
fn get_dir() -> PathBuf {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        Path::new(&args[1]).to_path_buf()
    } else {
        std::env::current_dir().unwrap()
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
