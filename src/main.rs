use std::fs::File;
use std::io::{BufRead, BufReader};

use day3::count_trees_part_one;

mod day1;
mod day2;
mod day3;

fn main() {
    let file = File::open("src/day3/input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let answer = count_trees_part_one(&lines);
    println!("Answer is: {}", answer);
}
