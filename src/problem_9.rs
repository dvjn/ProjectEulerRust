//! Special Pythagorean triplet

pub fn solve() -> u64 {
    for a in 1..=998 {
        for b in 1..=(999 - a) {
            let c = 1000 - a - b;
            if 1000 * c + a * b == 500000 {
                return a * b * c;
            }
        }
    }

    0
}
