/*
MIT License

Copyright (c) 2022 Philipp Schuster

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
//! # NFLZ (library)
//! Library to add leading zeros to ascending numbered file names. NFLZ stands for Numbered Files Leading Zeros.
//!
//! This library helps you to manage files inside your file system that belong to a set of ordered
//! files. An example are photos from a camera.
//!
//! [`NFLZAssistant`] is the main entry point into the library. Please check examples inside
//! the README or the repository.
//!
//! ## What it Does
//! **Content of some directory:**
//! On the left you see files that are inside a directory. The files may represent photos from one or
//! multiple cameras that you organized for example with Windows Explorer. `nflz` can add an appropriate
//! number of leading zeroes into the filename so that every trivial program can sort them correctly
//! by their apathetically order. In short: It ensures that chronological order equals the alphabetical
//! order.
//! ```text
//! paris (1).png   =>  paris (01).png
//! paris (2).png   =>  paris (02).png
//! ...
//! paris (12).png  =>  paris (12).png
//! ...
//! paris (n).png   =>  n digits => indicator for how many zeros to add
//! ```
//!
//! # Code Example
//! ```rust
//! use nflz::NFLZAssistant;
//!
//! /// Minimal example that renames all files in the given directory.
//! /// After the operation is done, all will include the same amount of digits
//! /// inside their number group inside the filename.
//! fn main() {
//!     const TEST_DIR: &str = ".test-resources";
//! #   // prevent that this doc test changes test resources covered by git
//! #   {
//! #       const TEST_DIR_SRC: &str = "test-resources";
//! #       if std::path::Path::is_dir(TEST_DIR.as_ref()) {
//! #           fs_extra::dir::remove(TEST_DIR).unwrap();
//! #       }
//! #       fs_extra::dir::copy(TEST_DIR_SRC, TEST_DIR, &fs_extra::dir::CopyOptions {
//! #          copy_inside: true, ..Default::default()
//! #       }).unwrap();
//! #   }
//!
//!     let assistant = NFLZAssistant::new(TEST_DIR).unwrap();
//!     dbg!(assistant.files_to_rename());
//!     // some files may already have the correct name
//!     dbg!(assistant.files_without_rename());
//!     if assistant.check_can_rename_all().is_ok() {
//!         assistant.rename_all().unwrap();
//!     }
//! }
//! ```
//!
//! # Library Design
//!

#![deny(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    // clippy::restriction,
    // clippy::pedantic
)]
// now allow a few rules which are denied by the above statement
// --> they are ridiculous and not necessary
#![allow(
    clippy::fallible_impl_from,
    clippy::needless_doctest_main,
    clippy::redundant_pub_crate,
    clippy::suboptimal_flops
)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(rustdoc::all)]
// #![allow(rustdoc::missing_doc_code_examples)]

/// See [`crate::error::NFLZError`].
pub use crate::error::NFLZError;

/// See [`crate::nflz::NFLZAssistant`].
pub use crate::nflz::NFLZAssistant;

mod error;
mod file_info;
mod fsutil;
mod math;
mod nflz;
