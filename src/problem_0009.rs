//! Special Pythagorean triplet

pub fn solve() -> u64 {
    product_of_pythgorean_triplet_with_sum(1000)
}

pub fn product_of_pythgorean_triplet_with_sum(sum: u64) -> u64 {
    let sum_squared = sum.pow(2);
    for a in 1..=(sum - 2) {
        for b in 1..=(sum - 1 - a) {
            let c = sum - a - b;
            if 2 * sum * c + 2 * a * b == sum_squared {
                return a * b * c;
            }
        }
    }

    panic!("Triplet not found!");
}

#[cfg(test)]
mod tests {
    use super::product_of_pythgorean_triplet_with_sum;

    #[test]
    fn given_example() {
        assert_eq!(product_of_pythgorean_triplet_with_sum(12), 60);
    }

    #[test]
    fn given_problem() {
        assert_eq!(product_of_pythgorean_triplet_with_sum(1000), 31875000);
    }
}
