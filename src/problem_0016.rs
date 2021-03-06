//! Power digit sum

pub fn solve() -> u64 {
    sum_of_digits_of_2s_power(1000)
}

pub fn sum_of_digits_of_2s_power(power: u64) -> u64 {
    let mut results: Vec<u8> = vec![1];
    let mut carry = 0;
    for _ in 1..=power {
        for digit in results.iter_mut() {
            let result = *digit * 2 + carry;
            *digit = result % 10;
            carry = result / 10;
        }
        while carry > 0 {
            results.push(carry % 10);
            carry /= 10;
        }
    }

    results.iter().map(|&digit| digit as u64).sum()
}

#[cfg(test)]
mod tests {
    use super::sum_of_digits_of_2s_power;

    #[test]
    fn given_example() {
        assert_eq!(sum_of_digits_of_2s_power(15), 26);
    }

    #[test]
    fn given_problem() {
        assert_eq!(sum_of_digits_of_2s_power(1000), 1366);
    }
}
