//! Lexicographic permutations

pub fn solve() -> u64 {
    get_nth_lexicographic_permutation(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 1_000_000)
}

fn get_number_of_permutations(length: usize) -> usize {
    (1..=length).fold(1, |x, y| x * y)
}

pub fn get_nth_lexicographic_permutation(digits: Vec<u64>, n: usize) -> u64 {
    let mut sorted_digits = digits.clone();
    sorted_digits.sort();

    get_nth_lexicographic_permutation_recursive(digits, n - 1)
        .iter()
        .fold(String::new(), |value, &digit| {
            value + digit.to_string().as_str()
        })
        .parse::<u64>()
        .unwrap()
}

fn get_nth_lexicographic_permutation_recursive(mut sorted_digits: Vec<u64>, n: usize) -> Vec<u64> {
    if n == 0 {
        return sorted_digits;
    }

    let number_of_sub_permutations = get_number_of_permutations(sorted_digits.len() - 1);
    let selected_digit_index = n / number_of_sub_permutations;
    let mut nth_lexicographic_permutation = vec![sorted_digits[selected_digit_index]];
    sorted_digits.remove(selected_digit_index);
    nth_lexicographic_permutation.extend(get_nth_lexicographic_permutation_recursive(
        sorted_digits,
        n % number_of_sub_permutations,
    ));

    nth_lexicographic_permutation
}

#[cfg(test)]
mod tests {
    use super::get_nth_lexicographic_permutation;

    #[test]
    fn given_example() {
        assert_eq!(get_nth_lexicographic_permutation(vec![0, 1, 2], 1), 012);
        assert_eq!(get_nth_lexicographic_permutation(vec![0, 1, 2], 2), 021);
        assert_eq!(get_nth_lexicographic_permutation(vec![0, 1, 2], 3), 102);
        assert_eq!(get_nth_lexicographic_permutation(vec![0, 1, 2], 4), 120);
        assert_eq!(get_nth_lexicographic_permutation(vec![0, 1, 2], 5), 201);
        assert_eq!(get_nth_lexicographic_permutation(vec![0, 1, 2], 6), 210);
    }

    #[test]
    fn given_problem() {
        assert_eq!(
            get_nth_lexicographic_permutation(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 1_000_000),
            2783915460
        );
    }
}
