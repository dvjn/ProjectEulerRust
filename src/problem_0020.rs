//! Factorial digit sum

pub fn solve() -> u64 {
    sum_of_digits_of_factorial(100)
}

pub fn sum_of_digits_of_factorial(number: u64) -> u64 {
    let mut results: Vec<u8> = vec![1];
    let mut carry = 0;
    for multiplicant in 1..=number {
        for digit in results.iter_mut() {
            let result = (*digit as u64) * multiplicant + carry;
            *digit = (result % 10) as u8;
            carry = result / 10;
        }
        while carry > 0 {
            results.push((carry % 10) as u8);
            carry /= 10;
        }
    }

    results.iter().map(|&digit| digit as u64).sum()
}

#[cfg(test)]
mod tests {
    use super::sum_of_digits_of_factorial;

    #[test]
    fn given_example() {
        assert_eq!(sum_of_digits_of_factorial(10), 27);
    }

    #[test]
    fn given_problem() {
        assert_eq!(sum_of_digits_of_factorial(100), 648);
    }
}
