//! Multiples of 3 and 5

pub fn solve() -> u64 {
    (3..1000).filter(|n| n % 3 == 0 || n % 5 == 0).sum::<u64>()
}
