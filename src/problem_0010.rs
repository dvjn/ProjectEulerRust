//! Summation of primes

pub fn solve() -> u64 {
    sum_of_primes(2_000_000)
}

pub fn sum_of_primes(below: u64) -> u64 {
    let mut is_prime: Vec<bool> = vec![true; (below + 1) as usize];
    let mut sum: u64 = 0;
    is_prime[0] = false;
    is_prime[1] = false;
    for number in 2usize..=(below as usize) {
        if is_prime[number] {
            sum += number as u64;
            for multiple in ((number + number)..=(below as usize)).step_by(number) {
                is_prime[multiple] = false;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::sum_of_primes;

    #[test]
    fn given_example() {
        assert_eq!(sum_of_primes(10), 17);
    }

    #[test]
    fn given_problem() {
        assert_eq!(sum_of_primes(2_000_000), 142913828922);
    }
}
