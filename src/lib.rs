#![macro_use]

#[macro_export]
macro_rules! mod_problems {($($problem:tt)*) => (::paste::paste! {
    $(mod [<problem_$problem>];)*
})}

#[macro_export]
macro_rules! solve_problem {($selected_problem:ident {$($problem:tt)*}) => (::paste::paste! {
    match $selected_problem {
        $($problem => [<problem_$problem>]::solve(),)*
        _ => panic!("Can't find the problem 🤷"),
    }
})}
