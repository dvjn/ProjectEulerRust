//! Digit fifth powers

pub fn solve() -> u64 {
    sum_of_nth_power(5)
}

pub fn sum_of_nth_power(power: usize) -> u64 {
    let mut sum = 0;

    for i in 2..354294 {
        let nth_power: u64 = i
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .map(|d| d.pow(power as u32))
            .sum();
        if nth_power == i {
            sum += i;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::sum_of_nth_power;

    #[test]
    fn given_example() {
        assert_eq!(sum_of_nth_power(4), 19316);
    }

    #[test]
    fn given_problem() {
        assert_eq!(sum_of_nth_power(5), 443839);
    }
}
