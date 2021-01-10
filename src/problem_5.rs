//! Smallest multiple

use std::cmp::max;
use std::collections::HashMap;

pub fn solve() -> i32 {
    (1..=20)
        .map(get_factor_map)
        .fold(HashMap::new(), combine_factor_maps)
        .iter()
        .fold(1i32, |number, (factor, frequency)| {
            number * (*factor as i32).pow(*frequency as u32)
        })
}

pub fn get_factor_map(n: u8) -> HashMap<u8, usize> {
    let mut factors_map: HashMap<u8, usize> = HashMap::new();

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
    factor_map_1: HashMap<u8, usize>,
    factor_map_2: HashMap<u8, usize>,
) -> HashMap<u8, usize> {
    let mut combined_factor_map: HashMap<u8, usize> = factor_map_1.clone();
    for (factor, frequency) in factor_map_2.iter() {
        match combined_factor_map.get_mut(&factor) {
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
