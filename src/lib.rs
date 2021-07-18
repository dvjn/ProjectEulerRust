#![macro_use]

use clipboard::{ClipboardContext, ClipboardProvider};
use paste::paste;
use std::error::Error;
use structopt::StructOpt;

#[macro_export]
macro_rules! include_problems {($($problem:tt)*) => (paste! {
    $(pub mod [<problem_$problem>];)*
    #[allow(clippy::zero_prefixed_literal)]
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

include_problems! {
    0001 0002 0003 0004 0005 0006 0007 0008 0009 0010
    0011 0012 0013 0014 0015 0016 0017 0018 0019 0020
    0021 0022 0023 0024
    0067
}

#[derive(Debug, StructOpt)]
#[structopt(name = "project_euler", about = "Let's solve Project Euler")]
pub enum ProjectEulerCli {
    Open { problem: u8 },
    Solve { problem: u8 },
}

// Helpers for cli

pub use webbrowser::open as open_url;
pub fn copy_to_clipboard(content: String) -> Result<(), Box<dyn Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(content)
}
