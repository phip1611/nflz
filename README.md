# nlfz - Numbered Filenames With Leading Zeros - CLI
**nlfz** is a CLI-Tool that helps you to add leading zeros to numbered filenames.

Build: &nbsp; [![Build Status](https://travis-ci.com/phip1611/malloc-log-lib.svg?branch=master)](https://travis-ci.com/phip1611/malloc-log-lib)


**Some Directory:**
```


paris (1).png   =>  paris (01).png
paris (2).png   =>  paris (02).png
...
paris (12).png  =>  paris (12).png
...
paris (n).png   =>  n digits => indicator for how many zeros to add 
```

## How it works
It doesn't need any parameters, it works in the pwd/cwd. It looks if all files has the same file extension
and all file's have the pattern `\([0-9]+\)` in them. Otherwise it will exit with a notice.

just compile it, add it to your PATH and then:
```
$ cd some/dir
$ nflz
```
## Background
If you select multiple files in Windows Explorer and rename them to the same name, Windows automatically
numbers all files for you. The downside is that there are no leading zeros. Other programs than Windows,
e.g. Google Drive, can't order the files properly without the leading zeros. Here comes my CLI into the game!
