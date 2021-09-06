//! Smallest multiple

use std::cmp::max;
use std::collections::HashMap;
use sugars::hmap;

pub fn solve() -> u64 {
    smallest_multiple(20)
}

pub fn smallest_multiple(upto: u64) -> u64 {
    (1..=upto)
        .map(get_factor_map)
        .fold(hmap!(), combine_factor_maps)
        .iter()
        .fold(1u64, |number, (factor, frequency)| {
            number * (*factor as u64).pow(*frequency as u32)
        })
}

pub fn get_factor_map(n: u64) -> HashMap<u64, usize> {
    let mut factors_map: HashMap<u64, usize> = hmap!();

    let mut number = n;
    let mut factor = 2;

    while number > 1 {
        if number % factor == 0 {
            number /= factor;
            match factors_map.get_mut(&factor) {
                Some(value) => {
                    *value += 1;
                }
                None => {
                    factors_map.insert(factor, 1usize);
                }
            }
        } else {
            factor += 1;
        }
    }

    factors_map
}

pub fn combine_factor_maps(
    factor_map_1: HashMap<u64, usize>,
    factor_map_2: HashMap<u64, usize>,
) -> HashMap<u64, usize> {
    let mut combined_factor_map: HashMap<u64, usize> = factor_map_1;
    for (factor, frequency) in factor_map_2.iter() {
        match combined_factor_map.get_mut(factor) {
            Some(combined_frequency) => {
                *combined_frequency = max(*combined_frequency, *frequency);
            }
            None => {
                combined_factor_map.insert(*factor, *frequency);
            }
        }
    }

    combined_factor_map
}

#[cfg(test)]
mod tests {
    use super::smallest_multiple;

    #[test]
    fn given_example() {
        assert_eq!(smallest_multiple(10), 2520);
    }

    #[test]
    fn given_problem() {
        assert_eq!(smallest_multiple(20), 232792560);
    }
}
