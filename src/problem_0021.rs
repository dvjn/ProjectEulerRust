//! Amicable numbers

use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};
use sugars::{hmap, hset, mutex};

pub fn solve() -> u64 {
    sum_of_amicable_numbers(10000)
}

lazy_static! {
    static ref PROPER_DIVISOR_SUMS: Mutex<HashMap<u64, u64>> = mutex!(hmap!());
}

pub fn get_proper_divisors(number: u64) -> HashSet<u64> {
    let mut divisors = hset!();
    let mut divisor = 1;
    let mut other_divisor = number;

    while divisor < other_divisor || divisor == 1 {
        if number % divisor == 0 {
            other_divisor = number / divisor;
            divisors.insert(divisor);
            divisors.insert(other_divisor);
        }
        divisor += 1;
    }

    divisors.remove(&number);

    divisors
}

pub fn get_proper_divisors_sum(number: u64) -> u64 {
    let proper_divisors_sum_option = PROPER_DIVISOR_SUMS.lock().unwrap().get(&number).cloned();
    return match proper_divisors_sum_option {
        Some(proper_divisors_sum) => proper_divisors_sum,
        None => {
            let proper_divisors_sum = get_proper_divisors(number).iter().sum();

            PROPER_DIVISOR_SUMS
                .lock()
                .unwrap()
                .insert(number, proper_divisors_sum);

            proper_divisors_sum
        }
    };
}

pub fn sum_of_amicable_numbers(under: u64) -> u64 {
    let mut amicable_numbers = hset!();

    for number in 2..=under {
        let proper_divisors_sum = get_proper_divisors_sum(number);

        if proper_divisors_sum < number {
            if let Some(amicable_pair_proper_divisors_sum) = PROPER_DIVISOR_SUMS
                .lock()
                .unwrap()
                .get(&proper_divisors_sum)
                .cloned()
            {
                if amicable_pair_proper_divisors_sum == number {
                    amicable_numbers.insert(number);
                    amicable_numbers.insert(proper_divisors_sum);
                }
            }
        }
    }

    amicable_numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::sum_of_amicable_numbers;

    #[test]
    fn given_problem() {
        assert_eq!(sum_of_amicable_numbers(10_000), 31626);
    }
}
