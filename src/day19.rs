use std::collections::HashMap;
use std::fs;

pub fn run() {
    part1();
    part2();
}

pub fn part1() {
    let input = fs::read_to_string("inputs/day19.txt").unwrap();
    let sections: Vec<_> = input.split("\n\n").collect();

    let rules: RuleSet = sections[0].lines().map(Rule::parse_line).collect();

    let num_valid = sections[1].lines().filter(|&m| check(m, &rules)).count();
    println!("day19.part1.solution = {}", num_valid);
}

pub fn part2() {
    let input = fs::read_to_string("inputs/day19.2.txt").unwrap();
    let sections: Vec<_> = input.split("\n\n").collect();

    let rules: RuleSet = sections[0].lines().map(Rule::parse_line).collect();

    let num_valid = sections[1].lines().filter(|&m| check(m, &rules)).count();
    println!("day19.part2.solution = {}", num_valid);
}

fn check(message: &str, rules: &RuleSet) -> bool {
    checkp(message, rules, 0)
        .iter()
        .filter(|&r| *r == "")
        .count()
        > 0
}

fn checkp<'a>(message: &'a str, rules: &'a RuleSet, rule: RuleId) -> Vec<&'a str> {
    match rules.get(&rule) {
        None => vec![],

        Some(Rule::Leaf(c)) => {
            if !message.is_empty() {
                vec![]
            } else if message[..1] == c[..] {
                vec![&message[1..]]
            } else {
                vec![]
            }
        }

        Some(Rule::Children(children)) => children
            .iter()
            .map(|child| {
                let mut routes: Vec<&'a str> = vec![message];

                for &rule in child {
                    routes = routes
                        .iter()
                        .map(|r| checkp(r, rules, rule))
                        .flatten()
                        .collect();
                }

                routes
            })
            .flatten()
            .collect(),
    }
}

type RuleId = u32;
type RuleSet = HashMap<RuleId, Rule>;

#[derive(Debug)]
enum Rule {
    Leaf(String),
    Children(Vec<Vec<RuleId>>),
}

impl Rule {
    fn parse_line(line: &str) -> (RuleId, Rule) {
        let mut parts = line.split(": ");

        let id: RuleId = parts.next().unwrap().parse().unwrap();

        let rhs = parts.next().unwrap();

        if rhs.contains('"') {
            (id, Rule::Leaf(rhs[1..2].to_string()))
        } else {
            (
                id,
                Rule::Children(
                    rhs.split('|')
                        .map(|p| p.split_whitespace().map(|c| c.parse().unwrap()).collect())
                        .collect(),
                ),
            )
        }
    }
}
