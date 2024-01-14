use std::error::Error;
use std::convert::TryFrom;

use crate::file::{read_file};

const NUM_ADJACENT_PARTS_FOR_GEAR_RATIO: u32 = 2;

pub fn problem03_part_1(input_file: &str) -> Result<(), Box<dyn Error>> {
    let engine_schematic = match parse_engine_schematic(input_file) {
        Ok(schematic) => schematic,
        Err(err) => return Err(err),
    };

    let sum_of_adjacent_parts: u32 = find_adjacent_part_numbers(&engine_schematic).iter().sum();
    println!("Sum of adjacent parts: {}", sum_of_adjacent_parts);

    return Ok(());
}

pub fn problem03_part_2(input_file: &str) -> Result<(), Box<dyn Error>> {
    let engine_schematic = match parse_engine_schematic(input_file) {
        Ok(schematic) => schematic,
        Err(err) => return Err(err),
    };

    let sum_of_gear_ratios: u32 = find_gear_ratios(&engine_schematic, NUM_ADJACENT_PARTS_FOR_GEAR_RATIO).iter().sum();
    println!("Sum of gear ratios: {}", sum_of_gear_ratios);

    return Ok(());
}

fn parse_engine_schematic(input_file: &str) -> Result<EngineSchematic, Box<dyn Error>> {
    let mut engine_schematic = EngineSchematic {
        part_numbers: Vec::new(),
        symbols: Vec::new(),
    };

    let input = read_file(input_file)?;
    let lines = input.lines();
    for (row, line) in lines.enumerate() {
        let mut processing_part_number = false;
        let mut part_number = 0;
        let mut start_column = 0;

        for (column, character) in line.char_indices() {
            if character.is_numeric() {
                // if we've reached a numeric digit, process a part number (either start to or continue to)
                let parsed_digit = character.to_digit(10).ok_or(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Character on line {} and column {} cannot be parsed to integer", row, column)
                )))?;

                if !processing_part_number {
                    // if we're not already processing a part number, store our starting column
                    start_column = column;
                }

                processing_part_number = true;
                part_number *= 10;
                part_number += parsed_digit;
            } else if processing_part_number {
                // if we're currently processing a part number but have reached a non-digit, then
                // we're at the end of the part number and need to add it to our struct
                engine_schematic.part_numbers.push(PartNumber {
                    number: part_number,
                    row: u32::try_from(row)?,
                    start_column: u32::try_from(start_column)?,
                    end_column: u32::try_from(column - 1)?, // subtract by one since the part number ended in the last iteration
                });

                // reset our variables
                processing_part_number = false;
                part_number = 0;
                start_column = 0;
            }

            if !character.is_numeric() && character != '.' {                
                engine_schematic.symbols.push(Symbol {
                    symbol: character,
                    row: u32::try_from(row)?,
                    column: u32::try_from(column)?,
                });
            }
        }

        // if we reached the end of the row and we're processing a part, make sure we add it to
        // our struct before proceeding
        if processing_part_number {
            engine_schematic.part_numbers.push(PartNumber {
                number: part_number,
                row: u32::try_from(row)?,
                start_column: u32::try_from(start_column)?,
                end_column: u32::try_from(line.chars().count() - 1)?, // the part number goes to the end of the row
            });
            // no need to reset variables - they're about to fall out of scope
        }
    }

    return Ok(engine_schematic);
}

fn find_adjacent_part_numbers(engine_schematic: &EngineSchematic) -> Vec<u32> {
    let mut adjacent_part_numbers = Vec::new();
    for part in engine_schematic.part_numbers.iter() {
        for symbol in engine_schematic.symbols.iter() {
            let left_boundary = if part.start_column == 0 { 0 } else { part.start_column - 1 };
            let right_boundary = part.end_column + 1;
            let top_boundary = if part.row == 0 { 0 } else { part.row - 1 };
            let bottom_boundary = part.row + 1;
            if symbol.column >= left_boundary && symbol.column <= right_boundary
                && symbol.row >= top_boundary && symbol.row <= bottom_boundary 
            {
                adjacent_part_numbers.push(part.number);
                break; // once we've added this part number, skip remaining symbols (to avoid double-adding)
            }
        }
    }

    return adjacent_part_numbers;
}



fn find_gear_ratios(engine_schematic: &EngineSchematic, num_adjacent_parts: u32) -> Vec<u32> {
    let mut gear_ratios = Vec::new();

    for symbol in engine_schematic.symbols.iter() {
        if symbol.symbol == '*' {
            // we multiply the gear ratio by each adjacent part.
            // For the first part, 1 * part = part. For each subsequent
            // part, the gear ratio increases multiplicatively as expected.
            let mut gear_ratio = 1;
            let mut current_adjacent_parts = 0;
            
            for part in engine_schematic.part_numbers.iter() {
                let left_boundary = if part.start_column == 0 { 0 } else { part.start_column - 1 };
                let right_boundary = part.end_column + 1;
                let top_boundary = if part.row == 0 { 0 } else { part.row - 1 };
                let bottom_boundary = part.row + 1;
                if symbol.column >= left_boundary && symbol.column <= right_boundary
                    && symbol.row >= top_boundary && symbol.row <= bottom_boundary 
                {
                    gear_ratio *= part.number;
                    current_adjacent_parts += 1;
                    
                    if current_adjacent_parts > num_adjacent_parts {
                        // short circuit if we've exceeded the # of adjacent parts we're looking for
                        break;
                    }
                }
            }

            if current_adjacent_parts == num_adjacent_parts {
                // if the number of adjacent parts found is equal to the number we're
                // looking for, add the gear ratio to the list
                gear_ratios.push(gear_ratio);
            }
        }
    }

    return gear_ratios;
}

#[derive(Debug)]
#[allow(unused)]
struct EngineSchematic {
    part_numbers: Vec<PartNumber>,
    symbols: Vec<Symbol>,
}

#[derive(Debug)]
#[allow(unused)]
struct PartNumber {
    number: u32,
    row: u32,
    start_column: u32,
    end_column: u32,
}

#[derive(Debug)]
#[allow(unused)]
struct Symbol {
    symbol: char,
    row: u32,
    column: u32,
}
