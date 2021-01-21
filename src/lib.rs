/*
MIT License

Copyright (c) 2021 Philipp Schuster

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
//! Library to handle "numbered (ascending) filenames with leading zeros"
//! It provides the functionality to fetch all files in a given directory,
//! that match the pattern `<prefix>(<number>)<suffix)`, like `paris (101).jpg`.
//! The library can provide you the following functionality:
//! * `paris (1).png` => `paris (001).png`
//! * `paris (2).png` => `paris (002).png`
//! * ...
//! * `paris (31).png` => `paris (031).png`
//! * `paris (100).png` => `paris (100).png`

/// See [`error::NFLZError`].
pub use crate::error::NFLZError;
/// See [`fsutil::get_matching_files`].
pub use crate::fsutil::get_matching_files;
/// See [`nflz::can_rename_all`].
pub use crate::nflz::can_rename_all;
/// See [`nflz::compute_rename_map`].
pub use crate::nflz::compute_rename_map;
/// See [`nflz::rename_all`].
pub use crate::nflz::rename_all;

mod error;
mod fsutil;
mod nflz;
mod parse;
