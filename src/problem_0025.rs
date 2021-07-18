//! 1000-digit Fibonacci number

pub fn solve() -> u64 {
    get_first_fibonacci_with_n_digits(1000)
}

pub fn get_first_fibonacci_with_n_digits(n: u64) -> u64 {
    // Formula Explanation: https://www.mathcha.io/editor/64yWvcZyC2rTLkpEYzC4E9olVF9JNDVkIddzq62
    ((n as f64 - 0.65051499784) / 0.20898764025).ceil() as u64
}

#[cfg(test)]
mod tests {
    use super::get_first_fibonacci_with_n_digits;

    #[test]
    fn given_example() {
        assert_eq!(get_first_fibonacci_with_n_digits(2), 7);
        assert_eq!(get_first_fibonacci_with_n_digits(3), 12);
    }

    #[test]
    fn given_problem() {
        assert_eq!(get_first_fibonacci_with_n_digits(1000), 4782);
    }
}
