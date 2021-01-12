//! Largest prime factor

pub fn solve() -> u64 {
    largest_prime_factor(600851475143)
}

pub fn largest_prime_factor(number: u64) -> u64 {
    let mut remainder = number;
    let mut factor = 2;

    while remainder > 1 {
        if remainder % factor == 0 {
            remainder /= factor;
        } else {
            factor += 1;
        }
    }

    factor
}
