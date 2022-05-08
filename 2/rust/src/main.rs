use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use rust::{determine_position, determine_position_with_aim_tracking, parse_directions};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .enumerate()
        .map(|(_, line)| line.unwrap())
        .collect();

    let directions = parse_directions(&lines);

    let position = determine_position(&directions);
    println!("Final position: {}", position);

    let position = determine_position_with_aim_tracking(&directions);
    println!("Final position with aim tracking: {}", position);

    Ok(())
}
