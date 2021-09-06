//! Number spiral diagonals

pub fn solve() -> u64 {
    sum_of_diagonals_of_spiral(1001)
}

pub fn sum_of_diagonals_of_spiral(size: u64) -> u64 {
    let mut sum = 1;
    let mut current_number = 1;

    for current_size in (3..=size).step_by(2) {
        for _ in 0..4 {
            current_number += current_size - 1;
            sum += current_number;
        }
    }

    1 + sum
}

#[cfg(test)]
mod tests {
    use super::sum_of_diagonals_of_spiral;

    #[test]
    fn given_example() {
        assert_eq!(sum_of_diagonals_of_spiral(5), 101);
    }

    #[test]
    fn given_problem() {
        assert_eq!(sum_of_diagonals_of_spiral(1001), 669171001);
    }
}
