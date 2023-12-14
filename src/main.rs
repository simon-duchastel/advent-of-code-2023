use std::io::{stdin, stdout, Write};
use crate::problems::problem01::{problem01};

mod problems;

fn main() {
    print!("Which problem would you like to solve? ");
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let problem: u32 = input.trim().parse().expect("You must enter a valid integer");
    match problem {
        1 => problem01(),
        _ => println!("Invalid problem entered - enter a number between 1-1"),
    }
}