//! Large sum

use lazy_static::lazy_static;
use std::fs;

const RADIX: u32 = 10;
const N_DIGITS: usize = 50;
const N_NUMBERS: usize = 100;

lazy_static! {
    static ref NUMBERS: Vec<Vec<u32>> = fs::read_to_string("inputs/problem_0013.txt")
        .expect("Cannot read input.")
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_digit(RADIX).unwrap())
                .rev()
                .collect()
        })
        .collect();
}

pub fn solve() -> u64 {
    digits_of_large_sum(10)
}

pub fn digits_of_large_sum(n: usize) -> u64 {
    let mut result_digits = vec![];
    let mut carry = 0;

    for digit in 0..N_DIGITS {
        let sum = (0..N_NUMBERS)
            .map(|number_index| NUMBERS[number_index][digit])
            .sum::<u32>()
            + carry;
        carry = sum / 10;
        result_digits.push(sum % 10);
    }

    while carry > 0 {
        result_digits.push(carry % 10);
        carry /= 10;
    }

    result_digits
        .iter()
        .rev()
        .take(n)
        .fold("".to_string(), |digits, digit| {
            digits + digit.to_string().as_str()
        })
        .parse::<u64>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::digits_of_large_sum;

    #[test]
    fn given_problem() {
        assert_eq!(digits_of_large_sum(10), 5537376230);
    }
}
