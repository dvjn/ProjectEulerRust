pub fn solve() -> i32 {
    (sum_of_natural_numbers(100).pow(2) - sum_of_squared_natural_numbers(100)) as i32
}

fn sum_of_natural_numbers(n: u32) -> u32 {
    (1..=n).sum()
}

fn sum_of_squared_natural_numbers(n: u32) -> u32 {
    (1..=n).map(|n| (n as u32).pow(2)).sum()
}
