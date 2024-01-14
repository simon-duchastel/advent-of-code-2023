use std::error::Error;

use crate::file::{read_file};

const RED_CUBES: u32 = 12;
const GREEN_CUBES: u32 = 13;
const BLUE_CUBES: u32 = 14;

pub fn problem02_part_1(input_file: &str) -> Result<(), Box<dyn Error>> {
    let input = read_file(input_file)?;
    let lines: Vec<&str> = input.lines().collect();
   
    let mut sum_of_ids: u32 = 0;
    for line in lines {
        let game_id = match get_game_id(&line) {
            Some(id) => id,
            None => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Unable to process game id")
                )));
            }
        };
        let line_without_id = line.split(": ").collect::<Vec<&str>>()[1];
        match is_game_possible(&line_without_id, RED_CUBES, GREEN_CUBES, BLUE_CUBES) {
            Ok(true) => {
                sum_of_ids += game_id;
            }
            Ok(false) => {} // intentional no-op - do not add the game sum if not possible
            Err(err) => {
                return Err(err);
            }
        }
    }

    println!("Sum of possible game IDs: {}", sum_of_ids);

    return Ok(());
}

pub fn problem02_part_2(input_file: &str) -> Result<(), Box<dyn Error>> {
    let input = read_file(input_file)?;
    let lines: Vec<&str> = input.lines().collect();
   
    let mut sum_of_power: u32 = 0;
    for line in lines {
        let line_without_id = line.split(": ").collect::<Vec<&str>>()[1];
        match get_max_cube_values_for_game(&line_without_id) {
            Ok((red, green, blue)) => {
                sum_of_power += red * green * blue;
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    println!("Sum of game powers: {}", sum_of_power);

    return Ok(());
}

fn get_game_id(game_input: &str) -> Option<u32> {
    let game_id_str = game_input.split(": ").collect::<Vec<&str>>().get(0)?.strip_prefix("Game ");
    match game_id_str {
        Some(id) => {
            return id.parse::<u32>().ok();
        }
        None => {
            return None;
        }
    }
}

fn is_game_possible(game_input: &str, red_cubes: u32, green_cubes: u32, blue_cubes: u32) -> Result<bool, Box<dyn Error>> {
    let rounds = game_input.split("; ").collect::<Vec<&str>>();
    for round in rounds {
        match get_cubes_in_round(&round) {
            Ok((red, green, blue)) => {
                if red > red_cubes || green > green_cubes || blue > blue_cubes {
                    // if any cubes exceed their expected amounts, the game is not possible
                    return Ok(false);
                }
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
    return Ok(true); // if we reach this point then no rounds are impossible, and thus the game is possible
}

// Get the maximum value for each of the red, green, and blue cubes for a game.
// The maximum values for the red, green, and blue cubes are returned in that order.
fn get_max_cube_values_for_game(game_input: &str) -> Result<(u32, u32, u32), Box<dyn Error>> {
    let rounds = game_input.split("; ").collect::<Vec<&str>>();
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    for round in rounds {
        match get_cubes_in_round(&round) {
            Ok((red, green, blue)) => {
                if red > max_red {
                    max_red = red
                }
                if green > max_green {
                    max_green = green
                }
                if blue > max_blue {
                    max_blue = blue
                }
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
    return Ok((max_red, max_green, max_blue));
}

// Given a string of the form "X blue, Y green, Z red" where the red, green, and blue
// numbers can come in any order and where X, Y, and Z are u32 integers, return the 
// number of red, green, and blue cubes (in that order).
fn get_cubes_in_round(round_input: &str) -> Result<(u32, u32, u32), Box<dyn Error>> {
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;
    for cube_str in round_input.split(", ") {
        if cube_str.ends_with(" red") {
            let without_suffix = if let Some(stripped) = cube_str.strip_suffix(" red") {
                stripped
            } else {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Unexpected error strippping ' red' suffix")
                )));
            };
            red = match without_suffix.parse::<u32>() {
                Ok(value) => value,
                Err(err) => return Err(Box::new(err)),
            }
        }
        if cube_str.ends_with(" green") {
            let without_suffix = if let Some(stripped) = cube_str.strip_suffix(" green") {
                stripped
            } else {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Unexpected error strippping ' green' suffix")
                )));
            };
            green = match without_suffix.parse::<u32>() {
                Ok(value) => value,
                Err(err) => return Err(Box::new(err)),
            }
        }
        if cube_str.ends_with(" blue") {
            let without_suffix = if let Some(stripped) = cube_str.strip_suffix(" blue") {
                stripped
            } else {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Unexpected error strippping ' blue' suffix")
                )));
            };
            blue = match without_suffix.parse::<u32>() {
                Ok(value) => value,
                Err(err) => return Err(Box::new(err)),
            }
        }
    }

    return Ok((red, green, blue));
}