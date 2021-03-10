use project_euler::{copy_to_clipboard, open_url, solve_problem, ProjectEulerCli};
use structopt::StructOpt;

fn main() {
    match ProjectEulerCli::from_args() {
        ProjectEulerCli::Open { problem } => {
            open_url(format!("https://projecteuler.net/problem={}", problem).as_str())
                .expect("Could not open browser 🙄");
        }
        ProjectEulerCli::Solve { problem } => {
            let answer = solve_problem(problem);
            copy_to_clipboard(answer.to_string()).expect("Failed to copy to clipboard 😤");
            println!("Answer to problem {} is {} 💡", problem, answer);
        }
    }
}
