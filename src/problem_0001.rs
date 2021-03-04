//! Multiples of 3 and 5

pub fn solve() -> u64 {
    sum_of_multiples_of_3_and_5(1000)
}

pub fn sum_of_multiples_of_3_and_5(below: u64) -> u64 {
    (3..below).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::sum_of_multiples_of_3_and_5;

    #[test]
    fn given_example() {
        assert_eq!(sum_of_multiples_of_3_and_5(10), 23);
    }

    #[test]
    fn given_problem() {
        assert_eq!(sum_of_multiples_of_3_and_5(1000), 233168);
    }
}
