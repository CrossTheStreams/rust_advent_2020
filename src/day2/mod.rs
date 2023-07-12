use regex::Regex;

// https://adventofcode.com/2020/day/2

pub fn count_valid_passwords_part_one(entries: &Vec<String>) -> i32 {
    let mut valid_count = 0;
    for entry in entries { 
        let min = first_num_from_entry(entry);
        let max: i32 = second_num_from_entry(entry);
        let character: char = char_from_entry(entry);
        let password: String = password_from_entry(entry);
        let character_count = password.chars().filter(|&c| {
            c == character
        }).count() as i32;
        println!("entry: {:}, character_count: {:}, min: {:}, max: {:}",
            entry,
            character_count,
            min,
            max
        );
        if character_count >= min && character_count <= max {
            println!("incrementing valid_count: {:}", valid_count);
            valid_count += 1;
        }
    } 
    return valid_count;
}

pub fn count_valid_passwords_part_two(entries: &Vec<String>) -> i32 {
    let mut valid_count = 0;
    for entry in entries {
        // These are 1-indexed
        let first_index = first_num_from_entry(entry) as usize;
        let second_index = second_num_from_entry(entry) as usize;
        let character: char = char_from_entry(entry);
        let password: String = password_from_entry(entry); 
        let first_char = password.chars().nth(first_index - 1).expect("No character at index");
        let second_char = password.chars().nth(second_index - 1).expect("No character at index");
        println!("entry: {:}, first_index: {:}, second_index: {:}, character: {:}, first_char: {:}, second_char: {:}",
            entry,
            first_index,
            second_index,
            character,
            first_char,
            second_char
        );
        // Either first_char or second_char must be the character, but not both
        if (first_char == character) ^ (second_char == character) {
            valid_count += 1;
        }
    } 
    valid_count
}

fn first_num_from_entry(entry: &String) -> i32 {
    let min: i32;
    let min_re = Regex::new(r"^\d+").unwrap();
    match min_re.captures(entry) {
       Some(cap) => {
            let matched_group = cap.get(0).unwrap().as_str();
            min = matched_group.parse().unwrap();
            return min;
       }
       None => {
            panic!("Pattern not found!")
       }
    }
}

fn second_num_from_entry(entry: &String) -> i32 {
    let max: i32;
    let max_re = Regex::new(r"(\d+)-(\d+)").unwrap();
    match max_re.captures(entry) {
        Some(cap) => {
             let matched_group = cap.get(2).unwrap().as_str();
             max = matched_group.parse().unwrap();
             return max;
        }
        None => {
             panic!("Pattern not found!")
        }
    }
}

fn char_from_entry(entry: &String) -> char {
    let character: char;
    let char_re = Regex::new(r"\s([a-z]{1})").unwrap();
    match char_re.captures(entry) {
        Some(cap) => {
            let matched_group = cap.get(0).unwrap().as_str().trim();
            character = matched_group.parse().unwrap();
            return character;
        }
        None => {
            panic!("Pattern not found!")
        }
    }
}

fn password_from_entry(entry: &String) -> String {
    let password: String;
    let password_re = Regex::new(r"[a-z]+$").unwrap();
    match password_re.captures(entry) {
        Some(cap) => {
            let matched_group = cap.get(0).unwrap().as_str();
            password = matched_group.parse().unwrap();
            return password;
        }
        None => {
            panic!("Pattern not found!")
        }
    }
}

#[test]
fn test_count_valid_passwords_part_one() {
    let passwords: Vec<String> = vec![
        String::from("1-3 a: abcde"),
        String::from("1-3 b: cdefg"),
        String::from("2-9 c: ccccccccc")
    ];
    assert_eq!(2, count_valid_passwords_part_one(&passwords));
}

#[test]
fn test_count_valid_passwords_part_two() {
    let passwords: Vec<String> = vec![
        String::from("1-3 a: abcde"),
        String::from("1-3 b: cdefg"),
        String::from("2-9 c: ccccccccc")
    ];
    assert_eq!(1, count_valid_passwords_part_two(&passwords));
}

#[test]
fn test_first_num_from_entry() {
    assert_eq!(1, first_num_from_entry(&String::from("1-3 a: abcde")));
    assert_eq!(1, first_num_from_entry(&String::from("1-3 b: cdefg")));
    assert_eq!(2, first_num_from_entry(&String::from("2-9 c: ccccccccc")));
}

#[test]
fn test_second_num_from_entry() {
    assert_eq!(3, second_num_from_entry(&String::from("1-3 a: abcde")));
    assert_eq!(3, second_num_from_entry(&String::from("1-3 b: cdefg")));
    assert_eq!(9, second_num_from_entry(&String::from("2-9 c: ccccccccc")));
}

#[test]
fn test_char_from_entry() {
    assert_eq!('a', char_from_entry(&String::from("1-3 a: abcde")));
    assert_eq!('b', char_from_entry(&String::from("1-3 b: cdefg")));
    assert_eq!('c', char_from_entry(&String::from("2-9 c: ccccccccc")));
}

#[test]
fn test_password_from_entry() {
    assert_eq!("abcde", password_from_entry(&String::from("1-3 a: abcde")));
    assert_eq!("cdefg", password_from_entry(&String::from("1-3 b: cdefg")));
    assert_eq!("ccccccccc", password_from_entry(&String::from("2-9 c: ccccccccc")));
}