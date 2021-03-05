//! Highly divisible triangular number

pub fn solve() -> u64 {
    first_triangular_number_with_over_n_factors(500)
}

pub fn get_n_factors(number: u64) -> usize {
    let mut n_factors = 0;
    let mut factor = 1;
    let mut other_factor = number;

    while factor < other_factor || factor == 1 {
        if number % factor == 0 {
            other_factor = number / factor;
            n_factors += if factor == other_factor { 1 } else { 2 };
        }
        factor += 1;
    }

    n_factors
}

pub fn first_triangular_number_with_over_n_factors(n: usize) -> u64 {
    let mut number = 1;
    let mut triangular_number = 1;
    loop {
        if get_n_factors(triangular_number) > n {
            return triangular_number;
        }
        number += 1;
        triangular_number += number;
    }
}

#[cfg(test)]
mod tests {
    use super::first_triangular_number_with_over_n_factors;

    #[test]
    fn given_example() {
        assert_eq!(first_triangular_number_with_over_n_factors(5), 28);
    }

    #[test]
    fn given_problem() {
        assert_eq!(first_triangular_number_with_over_n_factors(500), 76576500);
    }
}
