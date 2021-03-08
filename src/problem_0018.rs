//! Maximum path sum I

use std::{cmp::max, fs};

pub fn solve() -> u64 {
    maximum_path_sum(parse_triangle(
        fs::read_to_string("inputs/problem_0018.txt").expect("Cannot read input."),
    ))
}

pub fn parse_triangle(input: String) -> Vec<Vec<u64>> {
    input
        .split("\n")
        .map(|line| {
            line.split(" ")
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn maximum_path_sum(triangle: Vec<Vec<u64>>) -> u64 {
    let mut running_paths: Vec<u64> = triangle.last().unwrap().clone();
    for row in triangle.iter().rev().skip(1) {
        running_paths = row
            .iter()
            .zip(running_paths[..].windows(2))
            .map(|(new_route, running_path)| max(running_path[0], running_path[1]) + new_route)
            .collect();
    }

    running_paths[0]
}

#[cfg(test)]
mod tests {
    use super::{maximum_path_sum, parse_triangle};
    use std::fs;

    #[test]
    fn given_example() {
        assert_eq!(
            maximum_path_sum(parse_triangle(
                fs::read_to_string("inputs/problem_0018.test.txt").expect("Cannot read input.")
            )),
            23
        );
    }

    #[test]
    fn given_problem() {
        assert_eq!(
            maximum_path_sum(parse_triangle(
                fs::read_to_string("inputs/problem_0018.txt").expect("Cannot read input.")
            )),
            1074
        );
    }
}
