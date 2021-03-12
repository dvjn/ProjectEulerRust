//! Names scores

use lazy_static::lazy_static;
use std::fs;

pub fn solve() -> u64 {
    name_scores()
}

const ASCII_UPPER_ALPHBET_OFFSET: u64 = ('A' as u64) - 1;

lazy_static! {
    static ref NAMES: Vec<String> = {
        let mut names: Vec<String> = fs::read_to_string("inputs/problem_0022.txt")
            .expect("Cannot read input.")
            .split(',')
            .map(|name| {
                let mut chars = name.chars();
                chars.next();
                let mut name = chars.as_str().to_string();
                name.pop();
                name
            })
            .collect();
        names.sort();
        names
    };
}

pub fn get_name_worth(name: &str) -> u64 {
    name.to_ascii_uppercase()
        .chars()
        .map(|letter| (letter as u64) - ASCII_UPPER_ALPHBET_OFFSET)
        .sum()
}

pub fn name_scores() -> u64 {
    NAMES
        .iter()
        .enumerate()
        .map(|(index, name)| (index as u64 + 1) * get_name_worth(name.as_str()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{get_name_worth, name_scores};

    #[test]
    fn given_example() {
        assert_eq!(get_name_worth("COLIN"), 53);
    }

    #[test]
    fn given_problem() {
        assert_eq!(name_scores(), 871198282);
    }
}
