//! Reciprocal cycles

pub fn solve() -> u64 {
    get_longest_recurring_unit_fraction(1000)
}

fn get_recurring_length_of_unit_fraction(number: u64) -> usize {
    let mut remainders = vec![];
    let mut remainder = 1;
    while remainder != 0 && !remainders.contains(&remainder) {
        remainders.push(remainder);
        remainder = (remainder * 10) % number;
    }

    if remainder == 0 {
        return 0;
    }

    remainders.len()
        - remainders
            .iter()
            .position(|&digit| digit == remainder)
            .unwrap()
}

pub fn get_longest_recurring_unit_fraction(upto: u64) -> u64 {
    (1..=upto)
        .map(|number| (number, get_recurring_length_of_unit_fraction(number)))
        .max_by(|(_, recurring_length_1), (_, recurring_length_2)| {
            recurring_length_1.cmp(recurring_length_2)
        })
        .unwrap()
        .0 as u64
}

#[cfg(test)]
mod tests {
    use super::get_longest_recurring_unit_fraction;

    #[test]
    fn given_example() {
        assert_eq!(get_longest_recurring_unit_fraction(10), 7);
    }

    #[test]
    fn given_problem() {
        assert_eq!(get_longest_recurring_unit_fraction(1000), 983);
    }
}
