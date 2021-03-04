//! Largest palindrome product

pub fn solve() -> u64 {
    largest_palindrome_product(3)
}

pub fn largest_palindrome_product(digits: u32) -> u64 {
    let mut largest_product = 0;
    let lower_limit = 10u64.pow(digits - 1);
    let y_upper_limit = 10u64.pow(digits) - 1;
    let x_upper_limit = (lower_limit..=y_upper_limit)
        .rev()
        .find(|number| number % 11 == 0)
        .expect("No product is palindrome");

    for x in (lower_limit..=x_upper_limit).rev().step_by(11) {
        for y in lower_limit..=y_upper_limit {
            let product = x * y;
            if product > largest_product && is_palindrome(product) {
                largest_product = product;
            }
        }
    }

    largest_product as u64
}

pub fn is_palindrome(n: u64) -> bool {
    let mut number = n;
    let mut reversed = 0;

    while number > 0 {
        reversed = reversed * 10 + number % 10;
        number /= 10;
    }
    n == reversed
}

#[cfg(test)]
mod tests {
    use super::largest_palindrome_product;

    #[test]
    fn given_example() {
        assert_eq!(largest_palindrome_product(2), 9009);
    }

    #[test]
    fn given_problem() {
        assert_eq!(largest_palindrome_product(3), 906609);
    }
}
