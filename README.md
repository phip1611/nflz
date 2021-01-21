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
numbers all files for you. The downside is that there are no leading zeros. Other programs than Windows,
e.g. Google Drive, can't order the files properly without the leading zeros. Here comes my CLI into the game!
