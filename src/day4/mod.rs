use std::collections::HashMap;
use regex::bytes::Regex;

#[derive(Debug)]
struct Passport {
    byr: String,
    cid: String,
    ecl: String,
    eyr: String,
    hcl: String,
    hgt: String,
    iyr: String,
    pid: String,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: String::from(""),
            cid: String::from(""),
            ecl: String::from(""),
            eyr: String::from(""),
            hcl: String::from(""),
            hgt: String::from(""),
            iyr: String::from(""),
            pid: String::from(""),
        }
    }

    fn is_valid(&self) -> bool {
        return !self.byr.trim().is_empty() &&
        !self.ecl.trim().is_empty() &&
        !self.eyr.trim().is_empty() &&
        !self.hcl.trim().is_empty() &&
        !self.hgt.trim().is_empty() &&
        !self.iyr.trim().is_empty() && 
        !self.pid.trim().is_empty();
    }
}

#[test]
fn test_valid_passports() {
    let mut passport_with_cid = Passport::new();
    passport_with_cid.byr = "123".to_string();
    passport_with_cid.cid = "123".to_string();
    passport_with_cid.ecl = "123".to_string();
    passport_with_cid.eyr = "123".to_string();
    passport_with_cid.hcl = "123".to_string();
    passport_with_cid.hgt = "123".to_string();
    passport_with_cid.iyr = "123".to_string();
    passport_with_cid.pid = "123".to_string();

    assert!(passport_with_cid.is_valid());

    let mut password_no_cid = Passport::new();
    password_no_cid.byr = "123".to_string();
    password_no_cid.ecl = "123".to_string();
    password_no_cid.eyr = "123".to_string();
    password_no_cid.hcl = "123".to_string();
    password_no_cid.hgt = "123".to_string();
    password_no_cid.iyr = "123".to_string();
    password_no_cid.pid = "123".to_string();
    assert!(password_no_cid.is_valid());


    let mut password_no_byr = Passport::new();
    password_no_byr.cid = "123".to_string();
    password_no_byr.ecl = "123".to_string();
    password_no_byr.eyr = "123".to_string();
    password_no_byr.hcl = "123".to_string();
    password_no_byr.hgt = "123".to_string();
    password_no_byr.iyr = "123".to_string();
    password_no_byr.pid = "123".to_string();
    assert!(!password_no_byr.is_valid());
}
// Explain why this isn't counting valid passports
pub fn count_valid_passports(rows: Vec<String>) -> usize {
    let mut valid_count = 0;
    let mut passport = Passport::new();
    for row in rows {
        if row.trim().is_empty() {
            if passport.is_valid() {
                valid_count += 1;
            }
            passport = Passport::new();
        }
        let fields = fields_from_line(row.to_string()); 
        for (field_label, value) in fields {
            match field_label.as_str() {
                "byr" => passport.byr = value,
                "cid" => passport.cid = value,
                "ecl" => passport.ecl = value,
                "eyr" => passport.eyr = value,
                "hcl" => passport.hcl = value,
                "hgt" => passport.hgt = value,
                "iyr" => passport.iyr = value,
                "pid" => passport.pid = value,
                _ => panic!("Unexpected field label: {}", field_label)
            }
        }
    }
    // If the last passport is valid, we need to count it
    if passport.is_valid() {
        valid_count += 1;
    }
    valid_count
}

fn fields_from_line(line: String) -> HashMap<String, String> {
    let mut fields = HashMap::<String, String>::new();
    let field_regex = Regex::new(r"([a-z]{3}):(\S+)").unwrap();
    for matched in field_regex.find_iter(line.as_bytes()) {
        let matched_str: &str = std::str::from_utf8(matched.as_bytes()).unwrap();
        let split_str: Vec<&str> = matched_str.split(":").collect();
        fields.insert(split_str[0].to_string(), split_str[1].to_string());
    }
    return fields
}

#[test]
fn test_count_valid_passports() {
    let entries = vec![
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
        "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
        "".to_string(),
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
        "hcl:#cfa07d byr:1929".to_string(),
        "".to_string(),
        "hcl:#ae17e1 iyr:2013".to_string(),
        "eyr:2024".to_string(),
        "ecl:brn pid:760753108 byr:1931".to_string(),
        "hgt:179cm".to_string(),
        "".to_string(),
        "hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
        "iyr:2011 ecl:brn hgt:59in".to_string()
    ];
    assert_eq!(2, count_valid_passports(entries));
}

#[test]
fn test_fields_from_line() {
    assert_eq!(HashMap::from([("iyr".to_string(), "2013".to_string())]), fields_from_line("iyr:2013".to_string()));
    assert_eq!(HashMap::from([("hcl".to_string(), "#cfa07d".to_string())]), fields_from_line("hcl:#cfa07d".to_string()))
}