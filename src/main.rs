use std::env;
use std::process::ExitCode;

use crate::problems::problem01::{problem01_part_1, problem01_part_2};
use crate::problems::problem02::{problem02_part_1, problem02_part_2};

mod problems;
mod file;

fn main() -> ExitCode {
    let lower_problem_bound = 1;
    let upper_problem_bound = 2;

    let mut args = env::args().skip(1);
    let problem_number = match args.next() {
        Some(problem) => problem,
        None => {
            eprintln!("You must provide a valid problem number ({}-{}) as the first command line argument", lower_problem_bound, upper_problem_bound);
            return ExitCode::from(1);
        }
    };
    let input_file = match args.next() {
        Some(file) => file,
        None => {
            eprintln!("You must provide a valid filepath for the problem input as the second command line argument");
            return ExitCode::from(1);
        }
    };

    let result = match problem_number.trim() {
        "1a" => problem01_part_1(&input_file),
        "1b" => problem01_part_2(&input_file),
        "2a" => problem02_part_1(&input_file),
        "2b" => problem02_part_2(&input_file),
        trimmed => {
            eprintln!("Invalid problem entered - you entered '{}', but a number between {} and {} with either part a or b was expected", trimmed, lower_problem_bound, upper_problem_bound);
            return ExitCode::from(1);
        }
    };
    match result {
        Ok(_) => return ExitCode::from(0),
        Err(err) => {
            eprintln!("Error: {}", err);
            return ExitCode::from(1);
        }
    }
}