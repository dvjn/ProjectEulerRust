//! 10001st prime

use std::vec::Vec;

pub fn solve() -> u64 {
    let mut primes: Vec<u32> = vec![2];
    let mut n = 1;
    let mut current_number = 3;

    while n < 10_001 {
        if !primes.iter().any(|prime| current_number % prime == 0) {
            primes.push(current_number);
            n += 1;
        }
        current_number += 1;
    }

    primes.pop().unwrap() as u64
}
