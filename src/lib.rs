#![macro_use]

use clipboard::{ClipboardContext, ClipboardProvider};
use structopt::StructOpt;

#[macro_export]
macro_rules! include_problems {($($problem:tt)*) => (::paste::paste! {
    $(pub mod [<problem_$problem>];)*
    pub fn solve_problem(selected_problem: u8) -> u64 {
        match selected_problem {
            $($problem => {
                use $crate::[<problem_$problem>];
                [<problem_$problem>]::solve()
            },)*
            _ => panic!("Can't find the problem 🤷"),
        }
    }
})}

include_problems! {0001 0002 0003 0004 0005 0006 0007 0008 0009 0010 0011}

#[derive(Debug, StructOpt)]
#[structopt(name = "project_euler", about = "Let's solve Project Euler")]
pub enum ProjectEulerCli {
    Open { problem: u8 },
    Solve { problem: u8 },
}

pub fn copy_to_clipboard(content: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().expect("Failed to use clipboard 😤");
    ctx.set_contents(content)
        .expect("Failed to copy to clipboard 😤");
}
