//! Maximum path sum II

use crate::problem_0018::{maximum_path_sum, read_triangle};

pub fn solve() -> u64 {
    maximum_path_sum(read_triangle("inputs/problem_0067.txt"))
}

#[cfg(test)]
mod tests {
    use super::{maximum_path_sum, read_triangle};

    #[test]
    fn given_example() {
        assert_eq!(
            maximum_path_sum(read_triangle("inputs/problem_0067.test.txt")),
            23
        );
    }

    #[test]
    fn given_problem() {
        assert_eq!(
            maximum_path_sum(read_triangle("inputs/problem_0067.txt")),
            7273
        );
    }
}
