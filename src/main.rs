use std::fs::File;
use std::io::{BufRead, BufReader};

use day1::report_repair;

mod day1;

fn main() {
    let file = File::open("src/day1/input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let answer = report_repair(&lines);
    println!("Answer is: {}", answer);
}
