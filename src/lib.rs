#![macro_use]

use clipboard::{ClipboardContext, ClipboardProvider};
use structopt::StructOpt;

#[macro_export]
macro_rules! pub_mod_problems {($($problem:tt)*) => (::paste::paste! {
    $(pub mod [<problem_$problem>];)*
})}

#[macro_export]
macro_rules! solve_problem {($selected_problem:ident {$($problem:tt)*}) => (::paste::paste! {
    match $selected_problem {
        $($problem => {
            use $crate::[<problem_$problem>];
            [<problem_$problem>]::solve()
        },)*
        _ => panic!("Can't find the problem 🤷"),
    }
})}

pub_mod_problems! {1 2 3 4 5 6}

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
