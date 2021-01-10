pub fn solve() -> i32 {
    let mut largest_product = 0;

    for x in (100..=990).rev().step_by(11) {
        for y in 100..=999 {
            let product = x * y;
            if product > largest_product && is_palindrome(product) {
                largest_product = product;
            }
        }
    }

    largest_product as i32
}

fn is_palindrome(n: u32) -> bool {
    let mut number = n;
    let mut reversed = 0;

    while number > 0 {
        reversed = reversed * 10 + number % 10;
        number /= 10;
    }
    n == reversed
}
