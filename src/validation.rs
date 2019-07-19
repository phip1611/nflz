use crate::globals::REGEX;

// Performs checks to see if we can work with this filename
pub fn filename_is_valid(filename: &str) -> bool {
    if !matches_regex(filename) {
        return false;
    }
    true
}

// Most important check: see if there is something like "(123)" in the filename
fn matches_regex(filename: &str) -> bool {
    if !REGEX.is_match(filename) {
        return false;
    }
    true
}