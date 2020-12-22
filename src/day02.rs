use std::fs;

pub fn run() {
    part1();
    part2();
}

pub fn part1() {
    let entries = load_input("inputs/day2.txt");
    let num_valid = entries.iter().filter(|e| e.valid_part1()).count();
    println!("day02.part1.solution = {}", num_valid);
}

pub fn part2() {
    let entries = load_input("inputs/day2.txt");
    let num_valid = entries.iter().filter(|e| e.valid_part2()).count();
    println!("day02.part2.solution = {}", num_valid);
}

#[derive(Debug, PartialEq, Eq)]
pub struct Policy {
    pub letter: char,
    pub min: usize,
    pub max: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PasswordEntry {
    pub policy: Policy,
    pub password: String,
}

impl PasswordEntry {
    pub fn parse(line: &str) -> PasswordEntry {
        let parts: Vec<_> = line.split(|c| "- :".contains(c)).collect();

        PasswordEntry {
            policy: Policy {
                letter: parts[2].chars().next().unwrap(),
                min: parts[0].parse().unwrap(),
                max: parts[1].parse().unwrap(),
            },
            password: parts[4].to_string(),
        }
    }

    pub fn valid_part1(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|c| c == &self.policy.letter)
            .count();
        self.policy.min <= count && count <= self.policy.max
    }

    pub fn valid_part2(&self) -> bool {
        let first = self.password.chars().nth(self.policy.min - 1).unwrap() == self.policy.letter;
        let second = self.password.chars().nth(self.policy.max - 1).unwrap() == self.policy.letter;
        first != second
    }
}

pub fn load_input(filename: &str) -> Vec<PasswordEntry> {
    let content = fs::read_to_string(filename).unwrap();
    content.lines().map(PasswordEntry::parse).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_example_input() {
        let result = load_input("inputs/day2.example.txt");
        let expected = vec![
            PasswordEntry {
                policy: Policy {
                    letter: 'a',
                    min: 1,
                    max: 3,
                },
                password: "abcde".to_string(),
            },
            PasswordEntry {
                policy: Policy {
                    letter: 'b',
                    min: 1,
                    max: 3,
                },
                password: "cdefg".to_string(),
            },
            PasswordEntry {
                policy: Policy {
                    letter: 'c',
                    min: 2,
                    max: 9,
                },
                password: "ccccccccc".to_string(),
            },
        ];

        assert_eq!(result, expected);
    }
}
