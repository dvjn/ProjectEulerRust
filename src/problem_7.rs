//! 10001st prime

use std::vec::Vec;

pub fn solve() -> u64 {
    nth_prime(10_001)
}

pub fn nth_prime(n: usize) -> u64 {
    let mut primes: Vec<u64> = Vec::with_capacity(n);
    primes.push(2);
    let mut current_number = 3;

    while primes.len() < n {
        if !primes.iter().any(|prime| current_number % prime == 0) {
            primes.push(current_number);
        }
        current_number += 1;
    }

    primes.pop().unwrap()
}
