use std::{collections::HashMap};

// https://adventofcode.com/2020/day/1

pub fn report_repair_part_one(entries: &Vec<String>) -> i32 {
    let mut entries_to_index = HashMap::new();
    let mut answer = 0;
    for (idx, entry) in entries.iter().enumerate() {
        let number: i32 = entry.parse().expect("Should be valid number");
        let complement = 2020 - number;
        if entries_to_index.contains_key(&complement) {
            answer = number * complement;
            return answer;
        }
        entries_to_index.insert(number, idx as i32);
    } 
    answer
}

#[test]
fn test_report_repair_part_one() {
    let entries: Vec<String> = [
        "1721",
        "979",
        "366",
        "299",
        "675",
        "1456"
    ].iter().map(|&s| s.to_string()).collect();
    let answer = report_repair_part_one(&entries);
    assert_eq!(answer, 514579)
}

pub fn report_repair_part_two(entries: &Vec<String>) -> i32 {
    let mut sums_to_nums = HashMap::new();
    for (i, entry) in entries.iter().enumerate() {
        let num: i32 = entry.parse().expect("Should be valid number");
        for (j, entry) in entries.iter().enumerate() {
            if i == j {
                continue;
            }
            let other: i32 = entry.parse().expect("Should be valid number");
            let sum = num + other;
            let nums = (num, other);
            sums_to_nums.insert(sum, nums);
        }
    }
    let mut answer = 0;
    for entry in entries {
        let number: i32 = entry.parse().expect("Should be valid number");
        for (sum, nums) in &sums_to_nums {
            if (sum + number) == 2020 {
                let (ref a, ref b) = nums;
                answer = a * b * number;
                return answer;
            }
        }
    } 
    answer
}

#[test]
fn test_report_repair_part_two() {
    let entries: Vec<String> = [
        "1721",
        "979",
        "366",
        "299",
        "675",
        "1456"
    ].iter().map(|&s| s.to_string()).collect();
    let answer = report_repair_part_two(&entries);
    assert_eq!(answer, 241861950)
}
