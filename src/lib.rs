//! Library to handle "numbered (ascending) filenames with leading zeroes"
//! It provides the functionality to fetch all files in a given directory,
//! that match the pattern `<prefix>(<number>)<suffix)`, like `paris (101).jpg`.
//! The library can provide you the following functionality:
//! * `paris (1).png` => `paris (001).png`
//! * `paris (2).png` => `paris (002).png`
//! * ...
//! * `paris (31).png` => `paris (031).png`
//! * `paris (100).png` => `paris (100).png`

mod parse;
mod error;
mod fsutil;
mod nflz;

/// See [`error::NFLZError`].
pub use error::NFLZError;

/// See [`fsutil::get_matching_files`].
pub use fsutil::get_matching_files;

/// See [`nflz::compute_rename_map`].
pub use nflz::compute_rename_map;

/// See [`nflz::can_rename_all`].
pub use nflz::can_rename_all;

/// See [`nflz::rename_all`].
pub use nflz::rename_all;
