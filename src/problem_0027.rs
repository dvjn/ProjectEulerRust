//! Quadratic primes

use std::collections::HashSet;

pub fn solve() -> i64 {
    quadratic_coefficients_product_producing_max_primes(1000)
}

pub fn quadratic_coefficients_product_producing_max_primes(upto: u64) -> i64 {
    let primes: HashSet<i64> = {
        let mut primes = vec![2i64];
        let mut current_number = 3i64;
        while current_number <= (upto as i64) {
            if !primes.iter().any(|prime| current_number % prime == 0) {
                primes.push(current_number);
            }
            current_number += 2;
        }
        primes
    }
    .iter()
    .cloned()
    .collect();

    let mut max_primes: usize = 0;
    let mut max_coefficients: (i64, i64) = (0, 0);

    let a_range_start = if upto % 2 == 0 {
        -(upto as i64) + 1
    } else {
        -(upto as i64)
    };
    let a_range_end = upto as i64;

    for a in (a_range_start..=a_range_end).step_by(2) {
        for b in primes.iter().cloned() {
            let mut primes_count = 0;
            for n in 0i64.. {
                let solution = n.pow(2) + a * n + b;
                if !primes.contains(&solution) {
                    break;
                }
                primes_count += 1;
            }
            if primes_count > max_primes {
                max_primes = primes_count;
                max_coefficients = (a, b);
            }
        }
    }

    max_coefficients.0 * max_coefficients.1
}

#[cfg(test)]
mod tests {
    use super::quadratic_coefficients_product_producing_max_primes;

    #[test]
    fn given_example() {
        assert_eq!(
            quadratic_coefficients_product_producing_max_primes(1601),
            -126479
        );
    }

    #[test]
    fn given_problem() {
        assert_eq!(
            quadratic_coefficients_product_producing_max_primes(1000),
            -59231
        );
    }
}
