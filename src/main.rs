#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::day4::count_valid_passports;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let file = File::open("src/day4/input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.expect("Failed to read line")).collect();

    let valid_passports = count_valid_passports(lines);

    println!("Valid passports count is: {}", valid_passports);
}
