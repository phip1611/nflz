# nflz - Numbered Filenames With Leading Zeros - CLI + Library
**nflz** is a CLI-Tool + library that helps you to add leading zeros to numbered filenames in ascending order.

Build: &nbsp; [![Build Status](https://travis-ci.com/phip1611/nflz.svg?branch=main)](https://travis-ci.com/phip1611/nflz)

## What it does
**Content of some directory:**
```
paris (1).png   =>  paris (01).png
paris (2).png   =>  paris (02).png
...
paris (12).png  =>  paris (12).png
...
paris (n).png   =>  n digits => indicator for how many zeros to add 
```

### Note
NFLZ can't guarantee you 100% safety. Always make a backup first (:
But at my best will this should work when no catastrophic failure is happening.

## Install / use
### Rust library
Cargo.toml:
```
nflz = "<latest-version>"
```
### CLI tool
`$ cargo install nflz`

## How it works
It either works in pwd (present working dir) or in the directory passed as the first argument.

```
$ nflz
$ nflz <absolute or relative path to dir>
```

**`nflz` asks you for confirmation before it does any changes to your file system!**


## Background
If you select multiple files in Windows Explorer and rename them to the same name, Windows automatically
numbers all files for you in parentheses. The downside is that there are no leading zeros. Other programs than Windows,
e.g. Google Drive, can't order the files properly without the leading zeros. Here comes my CLI/lib into the game!

## Example output
```
NFLZ: using dir: "./test"
NFLZ: skipping file 'invalid (100) (19231).jpg' because: The filename 'invalid (100) (19231).jpg' must include exactly one numbered group.
NFLZ: Files that don't need a renaming:
  paris (734).jpg
NFLZ would rename files:
   paris (1).jpg => paris (001).jpg
   paris (2).jpg => paris (002).jpg
   paris (3).jpg => paris (003).jpg
   paris (4).jpg => paris (004).jpg
   paris (5).jpg => paris (005).jpg
   paris (6).jpg => paris (006).jpg
   paris (7).jpg => paris (007).jpg
   paris (8).jpg => paris (008).jpg
   paris (9).jpg => paris (009).jpg
  paris (10).jpg => paris (010).jpg

Please confirm with 'y' or abort with 'n'
NFLZ can't guarantee you 100% safety. Always make a backup first (:
But at my best will this should work when no catastrophic failure is happening.
```
