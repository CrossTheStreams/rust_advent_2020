use std::fs::File;
use std::io::{BufRead, BufReader};

use day2::count_valid_passwords;

mod day1;
mod day2;

fn main() {
    let file = File::open("src/day2/input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let answer = count_valid_passwords(&lines);
    println!("Answer is: {}", answer);
}
