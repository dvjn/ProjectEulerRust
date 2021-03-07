//! Largest product in a grid

use lazy_static::lazy_static;
use std::fs;

lazy_static! {
    static ref GRID: Vec<Vec<u64>> = fs::read_to_string("inputs/problem_0011.txt")
        .expect("Cannot read input.")
        .split('\n')
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse::<u64>().unwrap())
                .collect()
        })
        .collect();
}

pub fn solve() -> u64 {
    largest_product_in_grid(4)
}

macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::max($x, max!($($z),*)));
}

pub fn get_indices_product(indices: Vec<(usize, usize)>) -> u64 {
    let mut product = 1;
    for (row_index, col_index) in indices {
        match GRID.get(row_index) {
            Some(row) => match row.get(col_index) {
                Some(element) => {
                    product *= element;
                }
                None => return 0,
            },
            None => return 0,
        }
    }

    product
}

pub fn largest_product_in_grid(n: usize) -> u64 {
    let size: usize = 20;
    let mut max_product: u64 = 0;

    for row in 0..size {
        for col in 0..size {
            max_product = max!(
                max_product,
                get_indices_product((0..n).map(|i| (row, col + i)).collect()),
                get_indices_product((0..n).map(|i| (row + i, col)).collect()),
                get_indices_product((0..n).map(|i| (row + i, col + i)).collect()),
                get_indices_product((0..n).map(|i| (row + i, col.wrapping_sub(i))).collect())
            )
        }
    }

    max_product
}

#[cfg(test)]
mod tests {
    use super::largest_product_in_grid;

    #[test]
    fn given_problem() {
        assert_eq!(largest_product_in_grid(4), 70600674);
    }
}
