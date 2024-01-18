use std::error::Error;

use crate::file::{read_file};

pub fn problem05_part_1(input_file: &str) -> Result<(), Box<dyn Error>> {
    let almanac = parse_almanac(&input_file, false)?;

    if almanac.seeds.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Error parsing Almanac - must contain at least 1 seed")
        )));
    }

    let mut lowest_seed = almanac.seeds[0];
    let mut lowest_seed_location = get_seed_location(lowest_seed, &almanac)
        .ok_or(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Error getting seed location - could not map seed to location")
        )))?;
    for seed in &almanac.seeds {
        let next_seed_location = get_seed_location(*seed, &almanac)
            .ok_or(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error getting seed location - could not map seed to location")
            )))?;
        if next_seed_location < lowest_seed_location {
            lowest_seed = *seed;
            lowest_seed_location = next_seed_location;
        }
    }

    println!("Lowest seed location is {} for seed {}", lowest_seed_location, lowest_seed);

    Ok(())
}

pub fn problem05_part_2(input_file: &str) -> Result<(), Box<dyn Error>> {
    let almanac = parse_almanac(&input_file, true)?;

    if almanac.seeds.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Error parsing Almanac - must contain at least 1 seed")
        )));
    }

    let mut lowest_seed = almanac.seeds[0];
    let mut lowest_seed_location = get_seed_location(lowest_seed, &almanac)
        .ok_or(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Error getting seed location - could not map seed to location")
        )))?;
    let mut i = 0;
    for seed in &almanac.seeds {
        let next_seed_location = get_seed_location(*seed, &almanac)
            .ok_or(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error getting seed location - could not map seed to location")
            )))?;
        if next_seed_location < lowest_seed_location {
            lowest_seed = *seed;
            lowest_seed_location = next_seed_location;
        }
    }

    println!("Lowest seed location is {} for seed {}", lowest_seed_location, lowest_seed);

    Ok(())
}

fn parse_almanac(input_file: &str, seeds_as_pairs: bool) -> Result<Almanac, Box<dyn Error>> {
    let input = read_file(input_file)?;
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Error parsing Almanac - first line must be a list of seeds")
        )));
    }

    let mut almanac = Almanac {
        seeds: Vec::new(),
        seed_to_soil: Vec::new(),
        soil_to_fertilizer: Vec::new(),
        fertilizer_to_water: Vec::new(),
        water_to_light: Vec::new(),
        light_to_temperature: Vec::new(),
        temperature_to_humidity: Vec::new(),
        humidity_to_location: Vec::new(),
    };
    match lines[0].strip_prefix("seeds: ") {
        Some(stripped_line) if !seeds_as_pairs => almanac.seeds = parse_integer_element_line(stripped_line)?,
        Some(stripped_line) => {
            let raw_seeds = parse_integer_element_line(stripped_line)?;
            for s in 0..(raw_seeds.len()/2) {
                let start = raw_seeds[s*2];
                for seed in start..start+raw_seeds[s*2+1] {
                    almanac.seeds.push(seed);
                }
            }
        }
        None => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error parsing Almanac - first line must begin with 'seeds: '")
            )));
        }
    }

    let mut parsing: u8 = 0; // 0=nothing, 1=seed, 2=soil, 3=fertilizer, 4=water, 5=light, 6=temperature, 7=humidity
    for line in lines[1..].iter() {
        let trimmed_line = line.trim();
        match trimmed_line {
            "seed-to-soil map:" => parsing = 1,
            "soil-to-fertilizer map:" => parsing = 2,
            "fertilizer-to-water map:" => parsing = 3,
            "water-to-light map:" => parsing = 4,
            "light-to-temperature map:" => parsing = 5,
            "temperature-to-humidity map:" => parsing = 6,
            "humidity-to-location map:" => parsing = 7,
            l if l.is_empty() => continue, // skip empty lines
            _ => {
                let map_entry = parse_integer_tuple(trimmed_line)?;
                match parsing {
                    0 => {
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Error parsing Almanac - received non-empty line before getting map header")
                        )));
                    },
                    1 => almanac.seed_to_soil.push(map_entry),
                    2 => almanac.soil_to_fertilizer.push(map_entry),
                    3 => almanac.fertilizer_to_water.push(map_entry),
                    4 => almanac.water_to_light.push(map_entry),
                    5 => almanac.light_to_temperature.push(map_entry),
                    6 => almanac.temperature_to_humidity.push(map_entry),
                    7 => almanac.humidity_to_location.push(map_entry),
                    _ => {
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Unexpected error - parsing type outside of range")
                        )));
                    }
                }
            }
        }
    }

    Ok(almanac)
}

fn parse_integer_element_line(line: &str) -> Result<Vec<u64>, Box<dyn Error>> {
    let mut elements = Vec::new();
    let split_line = line.split(" ").map(|elem| elem.trim()).collect::<Vec<_>>();
    for elem in split_line {
        let parsed_element = elem.parse::<u64>()?;
        elements.push(parsed_element);
    }

    Ok(elements)
}

fn parse_integer_tuple(line: &str) -> Result<(u64, u64, u64), Box<dyn Error>> {
    let integer_vec = parse_integer_element_line(&line)?;
    if integer_vec.len() != 3 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Error parsing Almanac - map entry must include exactly 3 integers")
        )))
    }

    Ok((integer_vec[0], integer_vec[1], integer_vec[2]))
}

fn get_seed_location(seed: u64, almanac: &Almanac) -> Option<u64> {
    let mut key = seed;
    key = get_almanac_map(key, &almanac.seed_to_soil);
    key = get_almanac_map(key, &almanac.soil_to_fertilizer);
    key = get_almanac_map(key, &almanac.fertilizer_to_water);
    key = get_almanac_map(key, &almanac.water_to_light);
    key = get_almanac_map(key, &almanac.light_to_temperature);
    key = get_almanac_map(key, &almanac.temperature_to_humidity);
    key = get_almanac_map(key, &almanac.humidity_to_location);

    Some(key)
}

fn get_almanac_map(key: u64, map: &Vec<(u64, u64, u64)>) -> u64 {
    for tuple in map {
        let (value, tuple_key, range) = tuple;
        let upper_range = tuple_key + range;

        if tuple_key <= &key && key <= upper_range {
            return value + (key - tuple_key);
        }
    }

    key
}

#[derive(Debug)]
#[allow(unused)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Vec<(u64, u64, u64)>,
    soil_to_fertilizer: Vec<(u64, u64, u64)>,
    fertilizer_to_water: Vec<(u64, u64, u64)>,
    water_to_light: Vec<(u64, u64, u64)>,
    light_to_temperature: Vec<(u64, u64, u64)>,
    temperature_to_humidity: Vec<(u64, u64, u64)>,
    humidity_to_location: Vec<(u64, u64, u64)>,
}