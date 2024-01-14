use std::collections::HashMap;
use std::error::Error;

use crate::file::{read_file};

const VALUE_POWER: u32 = 2;

pub fn problem04_part_1(input_file: &str) -> Result<(), Box<dyn Error>> {
    let input = read_file(input_file)?;
    let lines: Vec<&str> = input.lines().collect();
   
    let mut sum_of_card_values: u32 = 0;
    for line in lines {
        let card = parse_card(&line)?;

        sum_of_card_values += card.value();
    }

    println!("Sum of card values: {}", sum_of_card_values);

    Ok(())
}

pub fn problem04_part_2(input_file: &str) -> Result<(), Box<dyn Error>> {
    let input = read_file(input_file)?;
    let lines: Vec<&str> = input.lines().collect();

    let mut num_cards: u32 = 0;
    let mut bonus_cards = HashMap::new();
    for line in lines {
        let card = parse_card(&line)?;
        let times_to_process = 1 + *bonus_cards.get(&card.id).get_or_insert(&0); // 1 for the original card + bonus cards

        for _ in 0..times_to_process {
            num_cards += 1;

            // for each nth win of this card, add a bonus card n further in the stack
            for bonus in 0..card.matching_numbers() {
                let bonus_id = card.id + bonus + 1;
                let old_value = *bonus_cards.get(&bonus_id).get_or_insert(&0); // offset by 1 to account for 0-index
                bonus_cards.insert(bonus_id, old_value + 1);
            }
        }
    }

    println!("Number of total cards: {}", num_cards);

    Ok(())
}

fn parse_card(line: &str) -> Result<Card, Box<dyn Error>> {
    let split_line = line.split(": ").collect::<Vec<&str>>();
    let id_section = split_line[0];
    let winning_numbers_section = split_line[1].split(" | ").collect::<Vec<&str>>()[0];
    let present_numbers_section = split_line[1].split(" | ").collect::<Vec<&str>>()[1];

    let id = match id_section.strip_prefix("Card ") {
        Some(id) => id.trim().parse::<u32>()?,
        None => return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Could not parse id for card")
        ))),
    };

    let winning_numbers = winning_numbers_section
        .split(" ")
        .map(|num| num.trim())
        .filter(|num| !num.is_empty())
        .map(|num| num.parse::<u32>())
        .filter(|num| num.is_ok())
        .map(|num| num.unwrap())
        .collect::<Vec<u32>>();

    let present_numbers = present_numbers_section
        .split(" ")
        .map(|num| num.trim())
        .filter(|num| !num.is_empty())
        .map(|num| num.parse::<u32>())
        .filter(|num| num.is_ok())
        .map(|num| num.unwrap())
        .collect::<Vec<u32>>();

    Ok(Card {
        id: id,
        winning_numbers: winning_numbers,
        present_numbers: present_numbers,
    })
}

#[derive(Debug)]
#[allow(unused)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    present_numbers: Vec<u32>,
}

impl Card {
    fn value(&self) -> u32 {
        let num = self.matching_numbers();
        if num > 0 {
            VALUE_POWER.pow(num - 1)
        } else {
            0
        }
    }

    fn matching_numbers(&self) -> u32 {
        let mut matching_numbers = 0;
        for number in self.present_numbers.iter() {
            if self.winning_numbers.contains(&number) {
                matching_numbers += 1;
            }
        }
        
        matching_numbers
    }
}