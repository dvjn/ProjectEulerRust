//! Largest product in a series

use lazy_static::lazy_static;
use std::fs;

const RADIX: u32 = 10;

lazy_static! {
    static ref DIGITS: Vec<u64> = fs::read_to_string("inputs/problem_0008.txt")
        .expect("Cannot read input.")
        .chars()
        .filter(|character| !character.is_whitespace())
        .map(|letter| letter.to_digit(RADIX).unwrap() as u64)
        .collect::<Vec<u64>>();
}

pub fn solve() -> u64 {
    largest_product_of_adjacent_digits(13)
}

pub fn largest_product_of_adjacent_digits(n: usize) -> u64 {
    DIGITS[..]
        .windows(n)
        .map(|window| window.iter().product())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::largest_product_of_adjacent_digits;

    #[test]
    fn given_example() {
        assert_eq!(largest_product_of_adjacent_digits(4), 5832);
    }

    #[test]
    fn given_problem() {
        assert_eq!(largest_product_of_adjacent_digits(13), 23514624000);
    }
}
