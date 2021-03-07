//! Number letter counts

use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

pub fn solve() -> u64 {
    number_letter_counts_upto(1000)
}

const ONES: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 9] = [
    "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const HUNDRED: &str = "hundred";
const AND: &str = "and";

const ORDERS: [&str; 6] = [
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

lazy_static! {
    static ref NUMBER_LETTER_COUNTS: Mutex<HashMap<u64, u64>> = {
        let mut m = HashMap::new();
        for (number, word) in ONES.iter().enumerate() {
            m.insert(number as u64, word.len() as u64);
        }
        for (number, word) in (10..100).step_by(10).zip(TENS.iter()) {
            m.insert(number as u64, word.len() as u64);
        }

        Mutex::new(m)
    };
}

pub fn get_number_letter_counts(number: u64) -> u64 {
    let number_letter_counts_option = NUMBER_LETTER_COUNTS.lock().unwrap().get(&number).cloned();

    return match number_letter_counts_option {
        Some(letter_counts) => letter_counts,
        None => {
            let letter_counts = match number {
                0..=19 => ONES[number as usize].len() as u64,
                20..=99 => {
                    if number % 10 == 0 {
                        TENS[(number / 10 - 1) as usize].len() as u64
                    } else {
                        TENS[(number / 10 - 1) as usize].len() as u64
                            + get_number_letter_counts(number % 10)
                    }
                }
                100..=999 => get_ordered_number_letter_counts(number, 100, HUNDRED),
                _ => {
                    let (divisor, order) = (1..=6)
                        .map(|power| 1000u64.pow(power))
                        .zip(ORDERS.iter())
                        .find(|&(exponent, _)| exponent > number / 1000)
                        .unwrap();

                    get_ordered_number_letter_counts(number, divisor, order)
                }
            };

            NUMBER_LETTER_COUNTS
                .lock()
                .unwrap()
                .insert(number, letter_counts);

            letter_counts
        }
    };
}

pub fn get_ordered_number_letter_counts(number: u64, divisor: u64, order: &str) -> u64 {
    match (number / divisor, number % divisor) {
        (upper, 0) => get_number_letter_counts(upper) + order.len() as u64,
        (upper, lower) => {
            get_number_letter_counts(upper)
                + if divisor == 100 {
                    order.len() + AND.len()
                } else {
                    order.len()
                } as u64
                + get_number_letter_counts(lower)
        }
    }
}

pub fn number_letter_counts_upto(upto: u64) -> u64 {
    (1..=upto).map(get_number_letter_counts).sum()
}

#[cfg(test)]
mod tests {
    use super::{get_number_letter_counts, number_letter_counts_upto};

    #[test]
    fn given_example() {
        assert_eq!(number_letter_counts_upto(5), 19);
        assert_eq!(get_number_letter_counts(342), 23);
        assert_eq!(get_number_letter_counts(115), 20);
    }

    #[test]
    fn given_problem() {
        assert_eq!(number_letter_counts_upto(1000), 21124);
    }
}
