//! Distinct powers

use crate::problem_0005::get_factor_map;
use itertools::Itertools;
use std::{collections::HashMap, hash::Hash};
use sugars::hset;

pub fn solve() -> u64 {
    terms_of_sequence_by_power(2, 100)
}

pub fn terms_of_sequence_by_power(from: u64, to: u64) -> u64 {
    let mut terms = hset! {};

    for a in from..=to {
        for b in from..=to {
            terms.insert(hash_factor_map(
                &get_factor_map(a)
                    .iter()
                    .map(|(&k, &v)| (k, (v as u64) * b))
                    .collect(),
            ));
        }
    }

    terms.len() as u64
}

fn hash_factor_map(hmap: &HashMap<u64, u64>) -> String {
    hmap.keys()
        .sorted()
        .map(|key| format!("{}^{}", key, hmap.get(key).unwrap()))
        .join("+")
}

pub fn match_hashmap<T: Eq + Hash, U: Eq>(map1: &HashMap<T, U>, map2: &HashMap<T, U>) -> bool {
    map1.len() == map2.len()
        && map1
            .keys()
            .all(|k| map2.contains_key(k) && map2.get(k).unwrap() == map1.get(k).unwrap())
}

#[cfg(test)]
mod tests {
    use super::terms_of_sequence_by_power;

    #[test]
    fn given_example() {
        assert_eq!(terms_of_sequence_by_power(2, 5), 15);
    }

    #[test]
    fn given_problem() {
        assert_eq!(terms_of_sequence_by_power(2, 100), 9183);
    }
}
