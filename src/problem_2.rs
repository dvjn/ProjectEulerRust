//! Even Fibonacci numbers

pub fn solve() -> i32 {
    let (mut a, mut b) = (1u32, 2u32);
    let mut sum = 0u32;
    while a < 4_000_000 {
        if a % 2 == 0 {
            sum += a;
        }
        b += a;
        a = b - a;
    }

    sum as i32
}
