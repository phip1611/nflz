# nflz - Numbered Filenames With Leading Zeros - CLI + Library
*CLI + Library to add leading zeros to ascending numbered file names. NFLZ stands for Numbered Files Leading Zeros.*

This library helps you to manage files inside your file system that belong to a set of ordered files. An example are
photos from a camera.

## What it Does
**Content of some directory:**
```
paris (1).png   =>  paris (01).png
paris (2).png   =>  paris (02).png
...
paris (12).png  =>  paris (12).png
...
paris (n).png   =>  n digits => indicator for how many zeros to add
```

## Install / How To Use
### Rust library
Cargo.toml:
```
nflz = "<latest-version>"
```

Minimal example:
```rust
use nflz::NFLZAssistant;

/// Minimal example that renames all files in the given directory.
/// After the operation is done, all will include the same amount of digits
/// inside their number group inside the filename.
fn main() {
    let assistant = NFLZAssistant::new("./test-resources").unwrap();
    dbg!(assistant.files_to_rename());
    // some files may already have the correct name
    dbg!(assistant.files_without_rename());
    if assistant.check_can_rename_all().is_ok() {
        assistant.rename_all().unwrap();
    }
}
```

Please also check out the docs on <https://docs.rs/nflz>.

### CLI tool
`$ cargo install nflz` \
It either works in the pwd (present working dir) or in the directory passed as the first argument.

```
$ nflz
$ nflz <absolute or relative path to dir>
```

**`nflz` asks you for confirmation before it does any changes to your file system!
However, always backup the files in another directory first to make sure nothing becomes inconsistent.**


## Background
If you select multiple files in Windows Explorer and rename them to the same name, Windows Explorer automatically
numbers all files for you in parentheses. The downside is that there are no leading zeros. Other programs than Windows,
e.g. Google Drive, can't order the files properly without the leading zeros. Here comes my CLI/lib into the game!

## Example output
```
NFLZ would not rename the following files:
  paris (734).jpg
NFLZ would rename the following files:
  paris (001).jpg           => paris (1).jpg
  paris (002).jpg           => paris (2).jpg
  paris (003).jpg           => paris (3).jpg
  paris (004).jpg           => paris (4).jpg
  paris (005).jpg           => paris (5).jpg
  paris (006).jpg           => paris (6).jpg
  paris (007).jpg           => paris (7).jpg
  paris (008).jpg           => paris (8).jpg
  paris (009).jpg           => paris (9).jpg
  paris (010).jpg           => paris (10).jpg

Please confirm with 'y' or abort with 'n'
  NFLZ can't guarantee you 100% safety. Always make a backup first (:
  But to the best of my knowledge this should work if no catastrophic failure occurs.
y
Successfully renamed 11 files.
```
