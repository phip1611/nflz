/// Returns how many digits a decimal number has.
pub fn digits (x: usize) -> usize {
    let x = (x + 1) as f64;
    x.log10().ceil() as usize
}
