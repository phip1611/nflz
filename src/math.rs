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
//! Math utility functions.

/// Returns the amount of digits of a number.
/// For example: 12345 => 5
pub fn count_digits_without_leading_zeroes(number: u64) -> u64 {
    let x = (number + 1) as f64;
    x.log10().ceil() as u64
}

#[cfg(test)]
mod tests {
    use crate::math::count_digits_without_leading_zeroes;

    #[test]
    fn test_count_digits_without_leading_zeroes() {
        assert_eq!(count_digits_without_leading_zeroes(0), 0);
        assert_eq!(count_digits_without_leading_zeroes(1), 1);
        assert_eq!(count_digits_without_leading_zeroes(9), 1);
        assert_eq!(count_digits_without_leading_zeroes(10), 2);
        assert_eq!(count_digits_without_leading_zeroes(999), 3);
    }
}
