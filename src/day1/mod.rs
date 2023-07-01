use std::{collections::HashMap};

// https://adventofcode.com/2020/day/1

pub fn report_repair(entries: &Vec<String>) -> i32 {
    // let entries_to_index: HashMap<&str, usize> = HashMap::new();
    let mut numbers: Vec<i32> = Vec::new();
    let mut entries_to_index = HashMap::new();
    let mut answer = 0;
    for (idx, entry) in entries.iter().enumerate() {
        let number = entry.parse().expect("Should be valid number");
        let complement = 2020 - number;
        if entries_to_index.contains_key(&complement) {
            answer = number * complement;
        }
        numbers.push(number);
        entries_to_index.insert(number, idx as i32);
    } 
    answer
}

#[test]
fn test_report_repair() {
    let entries: Vec<String> = [
        "1721",
        "979",
        "366",
        "299",
        "675",
        "1456"
    ].iter().map(|&s| s.to_string()).collect();
    let answer = report_repair(&entries);
    assert_eq!(answer, 514579)
}
