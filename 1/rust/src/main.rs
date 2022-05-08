use rust::{count_three_larger, count_larger_measurements};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let depths: Vec<i32> = args
        .iter()
        .skip(1)
        .map(|x| x.parse().expect("An integer was expected"))
        .collect();

    let result = count_larger_measurements(&depths);
    println!("{} depths with increasing measurements.", result);

    let result = count_three_larger(&depths);
    println!("{} depths with three increasing measurements.", result);
}
