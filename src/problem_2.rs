//! Even Fibonacci numbers

pub fn solve() -> u64 {
    sum_of_even_fibonacci_numbers(4_000_000)
}

pub fn sum_of_even_fibonacci_numbers(below: u64) -> u64 {
    let (mut a, mut b) = (1u64, 2u64);
    let mut sum = 0;
    while a < below {
        if a % 2 == 0 {
            sum += a;
        }
        b += a;
        a = b - a;
    }

    sum
}
