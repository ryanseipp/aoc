use std::fs::File;
use std::io::{BufRead, BufReader};

use bitvec::array::BitArray;
use bitvec::order::Msb0;
use rust::{get_power_consumption, get_life_support, parse_diagnostics};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let diagnostics: Vec<BitArray<u16, Msb0>> =
        parse_diagnostics(&reader.lines().map(|l| l.unwrap()).collect());

    let power_consumption = get_power_consumption(&diagnostics);
    println!("Power Consumption: {}", power_consumption);

    let life_support = get_life_support(12, &diagnostics);
    println!("Life Support: {}", life_support);
}
