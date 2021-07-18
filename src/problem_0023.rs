//! Non-abundant sums

use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use sugars::{hmap, hset, mutex};

pub fn solve() -> u64 {
    non_abundant_sums()
}

lazy_static! {
    static ref FACTORS: Mutex<HashMap<u64, HashSet<u64>>> = mutex!(hmap!());
    // generate abundant numbers till 28123
    static ref ABUNDANT_NUMBERS: Vec<u64> = generate_abundant_numbers(28123);
}

// "Fast Inverse Sqrt from Quake" Inverted
fn sqrt(x: f32) -> f32 {
    let i = x.to_bits();
    let i = 0x5f3759df - (i >> 1);
    let y = f32::from_bits(i);

    1.0 / (y * (1.5 - 0.5 * x * y * y))
}

fn get_factors(number: u64) -> HashSet<u64> {
    let factors_option = FACTORS.lock().unwrap().get(&number).cloned();
    return match factors_option {
        Some(factors) => factors,
        None => {
            let num_sqrt = sqrt(number as f32).floor() as u64;
            let mut factors = hset!(1, number);
            let mut factor = 1;

            while factor <= num_sqrt {
                factor += 1;
                if number % factor == 0 {
                    let other_factor = number / factor;
                    let other_factor_factors = get_factors(other_factor);
                    factors.insert(other_factor);
                    factors.extend(&other_factor_factors);
                    factors.extend(
                        other_factor_factors
                            .iter()
                            .map(|other_factor_factor| other_factor_factor * factor),
                    );
                }
            }

            if number != 1 {
                factors.remove(&number);
            }

            FACTORS
                .lock()
                .unwrap()
                .insert(number, factors.iter().map(|&num| num).collect());

            factors
        }
    };
}

fn sum_of_factors(number: u64) -> u64 {
    get_factors(number).iter().sum::<u64>()
}

fn generate_abundant_numbers(limit: u64) -> Vec<u64> {
    (12..=limit)
        .filter(|&number| sum_of_factors(number) > number)
        .collect()
}

pub fn non_abundant_sums() -> u64 {
    let mut abundant_sums = hset!();
    for abundant_number_1 in ABUNDANT_NUMBERS.iter() {
        for abundant_number_2 in ABUNDANT_NUMBERS.iter() {
            let sum = abundant_number_1 + abundant_number_2;
            if sum > 28123 {
                break;
            } else {
                abundant_sums.insert(sum);
            }
        }
    }

    (1..=28123)
        .filter(|&number| !abundant_sums.contains(&number))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{non_abundant_sums, sum_of_factors};

    #[test]
    fn twenty_eight_is_perfect() {
        assert_eq!(28, sum_of_factors(28));
    }

    #[test]
    fn given_problem() {
        assert_eq!(non_abundant_sums(), 4179871);
    }
}
