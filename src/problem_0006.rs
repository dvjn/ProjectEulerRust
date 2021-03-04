//! Sum square difference

pub fn solve() -> u64 {
    difference_of_sums_squared_and_squared_sums(100)
}

pub fn difference_of_sums_squared_and_squared_sums(upto: u64) -> u64 {
    (sum_of_natural_numbers(upto).pow(2) - sum_of_squared_natural_numbers(upto)) as u64
}

pub fn sum_of_natural_numbers(n: u64) -> u64 {
    (1..=n).sum()
}

pub fn sum_of_squared_natural_numbers(n: u64) -> u64 {
    (1..=n).map(|n| (n as u64).pow(2)).sum()
}

#[cfg(test)]
mod tests {
    use super::difference_of_sums_squared_and_squared_sums;

    #[test]
    fn given_example() {
        assert_eq!(difference_of_sums_squared_and_squared_sums(10), 2640);
    }

    #[test]
    fn given_problem() {
        assert_eq!(difference_of_sums_squared_and_squared_sums(100), 25164150);
    }
}
