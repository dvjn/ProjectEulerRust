use project_euler::{mod_problems, solve_problem};
use std::env;

mod_problems! {1}

fn main() {
    let selected_problem = env::args()
        .nth(1)
        .expect("Missing <problem_number> 🤦")
        .parse::<i32>()
        .expect("Non-integer <problem_number> 🤦");

    let answer = solve_problem! {selected_problem {1}};

    println!("Answer to problem {} is {}", selected_problem, answer);
}
