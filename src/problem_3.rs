pub fn solve() -> i32 {
    let mut number = 600851475143u64;
    let mut factor = 2;

    while number > 1 {
        if number % factor == 0 {
            number /= factor;
        } else {
            factor += 1;
        }
    }

    factor as i32
}
