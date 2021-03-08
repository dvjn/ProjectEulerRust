//! Maximum path sum II

use crate::problem_0018::{maximum_path_sum, parse_triangle};
use std::fs;

pub fn solve() -> u64 {
    maximum_path_sum(parse_triangle(
        fs::read_to_string("inputs/problem_0067.txt").expect("Cannot read input."),
    ))
}

#[cfg(test)]
mod tests {
    use super::{maximum_path_sum, parse_triangle};
    use std::fs;

    #[test]
    fn given_example() {
        assert_eq!(
            maximum_path_sum(parse_triangle(
                fs::read_to_string("inputs/problem_0067.test.txt").expect("Cannot read input.")
            )),
            23
        );
    }

    #[test]
    fn given_problem() {
        assert_eq!(
            maximum_path_sum(parse_triangle(
                fs::read_to_string("inputs/problem_0067.txt").expect("Cannot read input.")
            )),
            7273
        );
    }
}
