use regex::Regex;

lazy_static!(
    // RegEx that sees if we have a numbered filename; note the parentheses around
    // with them we have a capturing group that we can use with REGEX.capture(str)
    pub static ref REGEX: Regex = Regex::new(r"(\([0-9]+\))").expect("RegEx can't be created!");
);
