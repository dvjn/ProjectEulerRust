//! Longest Collatz sequence

use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

pub fn solve() -> u64 {
    longest_collatz_sequence_below(1_000_000)
}

lazy_static! {
    static ref COLLATZ_LENGTHS: Mutex<HashMap<u64, usize>> = {
        let mut m = HashMap::new();
        m.insert(1, 1);
        Mutex::new(m)
    };
}

pub fn get_collatz_length(number: u64) -> usize {
    let length_option = COLLATZ_LENGTHS.lock().unwrap().get(&number).cloned();
    return match length_option {
        Some(length) => length,
        None => {
            let length = get_collatz_length(if number % 2 == 0 {
                number / 2
            } else {
                3 * number + 1
            }) + 1;
            COLLATZ_LENGTHS.lock().unwrap().insert(number, length);
            length
        }
    };
}

pub fn longest_collatz_sequence_below(below: u64) -> u64 {
    (1..=below)
        .map(|number| (number, get_collatz_length(number)))
        .fold((1, 1), |(best_number, best_length), (number, length)| {
            if length > best_length {
                (number, length)
            } else {
                (best_number, best_length)
            }
        })
        .0 as u64
}

#[cfg(test)]
mod tests {
    use super::{get_collatz_length, longest_collatz_sequence_below};

    #[test]
    fn given_example() {
        assert_eq!(get_collatz_length(13), 10);
    }

    #[test]
    fn given_problem() {
        assert_eq!(longest_collatz_sequence_below(1_000_000), 837799);
    }
}
