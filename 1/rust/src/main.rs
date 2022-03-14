// use rust::count_larger_measurements;
use rust::count_three_larger;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let depths: Vec<i32> = args
        .iter()
        .skip(1)
        .map(|x| x.parse().expect("An integer was expected"))
        .collect();

    // let resunt = count_larger_measurements(&depths);
    let result = count_three_larger(&depths);

    println!("{} depths with increasing measurements.", result);
}
