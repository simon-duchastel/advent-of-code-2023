use std::error::Error;

use crate::file::{read_file};

pub fn problem01_part_1(input_file: &str) -> Result<(), Box<dyn Error>> {
    let input = read_file(input_file)?;
    let lines: Vec<&str> = input.lines().collect();

    let mut total_sum: u32 = 0;
    for line in lines {
        let first_digit = if let Some(first_digit) = line.chars().find(|&c| c.is_numeric()) {
            first_digit
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Improper input file format - all lines must contain at least 2 digits",
            )));
        };
        
        let last_digit = if let Some(last_digit) = line.chars().rev().find(|&c| c.is_numeric()) {
            last_digit
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Improper input file format - all lines must contain at least 2 digits",
            )));
        };

        let mut combined = first_digit.to_string();
        combined.push(last_digit);
        total_sum += combined.parse::<u32>().unwrap();
    }

    println!("Sum of calibration values: {}", total_sum);

    return Ok(());
}

pub fn problem01_part_2(input_file: &str) -> Result<(), Box<dyn Error>> {
    let input = read_file(input_file)?;
    let lines: Vec<&str> = input.lines().collect();

    let mut total_sum: u32 = 0;
    for line in lines {
        let first_digit = if let Some(first_digit) = find_first_digit(&line, false) {
            first_digit
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Improper input file format - all lines must contain at least 2 digits",
            )));
        };

        let last_digit = if let Some(last_digit) = find_first_digit(&line, true) {
            last_digit
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Improper input file format - all lines must contain at least 2 digits",
            )));
        };

        let mut combined = first_digit.to_string();
        combined.push(last_digit);
        total_sum += combined.parse::<u32>().unwrap();
    }

    println!("Sum of calibration values: {}", total_sum);

    return Ok(());
}

fn find_first_digit(input: &str, backwards: bool) -> Option<char> {
    let range: Box<dyn Iterator<Item = usize>> = if backwards {
        Box::new((0..input.len()).rev())
    } else {
        Box::new(0..input.len())
    };
    for i in range {
        match input.chars().nth(i) {
            Some(character) if character.is_numeric() => {
                return Some(character);
            }
            Some('z') => {
                if starts_with_0(&input[i..]) {
                    return Some('0');
                }
            }
            Some('o') => {
                if starts_with_1(&input[i..]) {
                    return Some('1');
                }
            }
            Some('t') => {
                if starts_with_2(&input[i..]) {
                    return Some('2');
                } else if starts_with_3(&input[i..]) {
                    return Some('3');
                }
            }
            Some('f') => {
                if starts_with_4(&input[i..]) {
                    return Some('4');
                } else if starts_with_5(&input[i..]) {
                    return Some('5');
                }
            }
            Some('s') => {
                if starts_with_6(&input[i..]) {
                    return Some('6');
                } else if starts_with_7(&input[i..]) {
                    return Some('7');
                }
            }
            Some('e') => {
                if starts_with_8(&input[i..]) {
                    return Some('8');
                }
            }
            Some('n') => {
                if starts_with_9(&input[i..]) {
                    return Some('9');
                }
            }
            Some(_) => (), // do nothing - proceed with loop
            None => {
                return None; // end of string - no digits found
            }
        }
    }

    return None;
}

fn starts_with_0(input: &str) -> bool {
    return input.chars().count() >= 4 && &input[0..4] == "zero"  
}

fn starts_with_1(input: &str) -> bool {
    return input.chars().count() >= 3 && &input[0..3] == "one"
}

fn starts_with_2(input: &str) -> bool {
    return input.chars().count() >= 3 && &input[0..3] == "two"
}

fn starts_with_3(input: &str) -> bool {
    return input.chars().count() >= 5 && &input[0..5] == "three"
}

fn starts_with_4(input: &str) -> bool {
    return input.chars().count() >= 4 && &input[0..4] == "four"
}

fn starts_with_5(input: &str) -> bool {
    return input.chars().count() >= 4 && &input[0..4] == "five"
}

fn starts_with_6(input: &str) -> bool {
    return input.chars().count() >= 3 && &input[0..3] == "six"
}

fn starts_with_7(input: &str) -> bool {
    return input.chars().count() >= 5 && &input[0..5] == "seven"
}

fn starts_with_8(input: &str) -> bool {
    return input.chars().count() >= 5 && &input[0..5] == "eight"
}

fn starts_with_9(input: &str) -> bool {
    return input.chars().count() >= 4 && &input[0..4] == "nine"
}