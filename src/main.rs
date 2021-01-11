use project_euler::{copy_to_clipboard, solve_problem, ProjectEulerCli};
use structopt::StructOpt;
use webbrowser;

fn main() {
    match ProjectEulerCli::from_args() {
        ProjectEulerCli::Open { problem } => {
            webbrowser::open(format!("https://projecteuler.net/problem={}", problem).as_str())
                .expect("Could not open browser 🙄");
        }
        ProjectEulerCli::Solve { problem } => {
            let answer = solve_problem(problem);
            copy_to_clipboard(answer.to_string());
            println!("Answer to problem {} is {} 💡", problem, answer);
        }
    }
}
