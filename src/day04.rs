use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub fn run() {
    part1();
    part2();
}

#[derive(Debug)]
struct PassportData {
    pub pairs: HashMap<String, String>,
}

const REQUIRED_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

impl PassportData {
    pub fn parse(raw: &str) -> PassportData {
        let parts = raw.split_ascii_whitespace();
        let mut passport = PassportData {
            pairs: HashMap::new(),
        };

        for pair in parts {
            let pair = pair.to_string();
            let split_idx = pair.find(':').unwrap();
            let (key, value) = pair.split_at(split_idx);
            passport
                .pairs
                .insert(key.to_string(), value[1..].to_string());
        }

        passport
    }

    pub fn keys_present(&self) -> bool {
        for key in &REQUIRED_KEYS {
            if !self.pairs.contains_key(&key.to_string()) {
                return false;
            }
        }
        true
    }

    pub fn valid(&self) -> bool {
        self.keys_present()
            && self.valid_byr()
            && self.valid_iyr()
            && self.valid_eyr()
            && self.valid_hgt()
            && self.valid_hcl()
            && self.valid_ecl()
            && self.valid_pid()
    }

    fn valid_byr(&self) -> bool {
        match self.pairs["byr"].parse() {
            Ok(year) => 1920 <= year && year <= 2002,
            Err(_) => false,
        }
    }

    fn valid_iyr(&self) -> bool {
        match self.pairs["iyr"].parse() {
            Ok(year) => 2010 <= year && year <= 2020,
            Err(_) => false,
        }
    }

    fn valid_eyr(&self) -> bool {
        match self.pairs["eyr"].parse() {
            Ok(year) => 2020 <= year && year <= 2030,
            Err(_) => false,
        }
    }

    fn valid_hgt(&self) -> bool {
        let hgt = self.pairs["hgt"].clone();
        let re = Regex::new(r"(\d+)([[:alpha:]]+)").unwrap();
        match re.captures(&hgt) {
            Some(cap) => {
                let hgt: u32 = cap[1].parse().unwrap();
                let unit: &str = &cap[2];
                (unit == "in" && 59 <= hgt && hgt <= 76)
                    || (unit == "cm" && 150 <= hgt && hgt <= 193)
            }
            None => false,
        }
    }

    fn valid_hcl(&self) -> bool {
        let hcl = self.pairs["hcl"].clone();
        Regex::new(r"#[0-9a-f]{6}").unwrap().is_match(&hcl)
    }

    fn valid_ecl(&self) -> bool {
        let colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        colors.contains(&self.pairs["ecl"].as_str())
    }

    fn valid_pid(&self) -> bool {
        let pid = self.pairs["pid"].clone();
        Regex::new(r"^[0-9]{9}$").unwrap().is_match(pid.as_str())
    }
}

pub fn part1() {
    let records: Vec<_> = fs::read_to_string("inputs/day4.txt")
        .unwrap()
        .split("\n\n")
        .map(PassportData::parse)
        .collect();

    let valid_records: Vec<_> = records.iter().filter(|x| x.keys_present()).collect();

    println!("day04.part1.solution = {}", valid_records.len());
}

pub fn part2() {
    let records: Vec<_> = fs::read_to_string("inputs/day4.txt")
        .unwrap()
        .split("\n\n")
        .map(PassportData::parse)
        .collect();

    let valid_records: Vec<_> = records.iter().filter(|x| x.valid()).collect();

    println!("day04.part2.solution = {}", valid_records.len());
}
