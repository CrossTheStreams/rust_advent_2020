use std::fs::File;
use std::io::{BufRead, BufReader};

use day3::count_trees;

mod day1;
mod day2;
mod day3;

fn main() {
    let file = File::open("src/day3/input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let mut answer = count_trees(&lines, (1, 1));
    answer = answer * count_trees(&lines, (3, 1));
    answer = answer * count_trees(&lines, (5, 1));
    answer = answer * count_trees(&lines, (7, 1));
    answer = answer * count_trees(&lines, (1, 2));
    println!("Answer is: {}", answer);
}
