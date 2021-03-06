//! Lattice paths

use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

pub fn solve() -> u64 {
    number_of_square_lattice_paths(20)
}

lazy_static! {
    static ref PATHS: Mutex<HashMap<(u64, u64), u64>> = {
        let mut m = HashMap::new();
        m.insert((0, 0), 1);
        Mutex::new(m)
    };
}

pub fn number_of_square_lattice_paths(grid_size: u64) -> u64 {
    number_of_lattice_paths(grid_size, grid_size)
}

pub fn number_of_lattice_paths(grid_x: u64, grid_y: u64) -> u64 {
    let paths_option = PATHS.lock().unwrap().get(&(grid_x, grid_y)).cloned();
    return match paths_option {
        Some(paths) => paths,
        None => {
            let paths = if grid_x > 0 {
                number_of_lattice_paths(grid_x.saturating_sub(1), grid_y)
            } else {
                0
            } + if grid_y > 0 {
                number_of_lattice_paths(grid_x, grid_y.saturating_sub(1))
            } else {
                0
            };

            PATHS.lock().unwrap().insert((grid_x, grid_y), paths);

            paths
        }
    } as u64;
}

#[cfg(test)]
mod tests {
    use super::number_of_square_lattice_paths;

    #[test]
    fn given_example() {
        assert_eq!(number_of_square_lattice_paths(2), 6);
    }

    #[test]
    fn given_problem() {
        assert_eq!(number_of_square_lattice_paths(20), 137846528820);
    }
}
