use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

pub fn part1() {
    let input = fs::read_to_string("inputs/day16.txt").unwrap();

    let sections: Vec<&str> = input.split("\n\n").collect();

    let mut ranges = vec![];

    for line in sections[0].lines() {
        let (_name, mut range_pair) = parse_field_line(line);
        ranges.append(&mut range_pair);
    }

    let mut ticket_lines = sections[2].lines();
    ticket_lines.next();
    let other_tickets: Vec<_> = ticket_lines.map(parse_ticket).collect();

    let mut sum_invalid_fields = 0;

    for ticket in other_tickets {
        for field in ticket {
            if !valid_field(field, &ranges) {
                sum_invalid_fields += field;
            }
        }
    }

    println!("day16.part1.solution = {}", sum_invalid_fields);
}

pub fn part2() {
    let input = fs::read_to_string("inputs/day16.txt").unwrap();

    let sections: Vec<&str> = input.split("\n\n").collect();

    let mut ranges = vec![];
    let mut rules = vec![];

    for line in sections[0].lines() {
        let (name, range_pair) = parse_field_line(line);
        for each in &range_pair {
            ranges.push(*each);
        }
        rules.push((name, range_pair));
    }

    let mut ticket_lines = sections[2].lines();
    ticket_lines.next();

    let valid_tickets: Vec<_> = ticket_lines
        .map(parse_ticket)
        .filter(|ticket| valid_ticket(&ticket, &ranges))
        .collect();
    let num_columns = valid_tickets[0].len();

    let columns: Vec<Vec<u64>> = (0..num_columns)
        .map(|col| valid_tickets.iter().map(|t| t[col]).collect())
        .collect();

    let mut available: HashSet<usize> = (0..num_columns).collect();
    let mut mapping: HashMap<&str, usize> = HashMap::new();

    // i apologize for my sins. this could be done so much better, but i'm doing the stupid way
    // because honestly, it's late and i have other things i need to do, and this is "just for
    // fun".
    while !available.is_empty() {
        println!("{:#?}", available);
        for (name, ranges) in &rules {
            let matching: Vec<_> = columns
                .iter()
                .enumerate()
                .filter(|(idx, col)| {
                    available.contains(idx) && col.iter().all(|field| valid_field(*field, &ranges))
                })
                .collect();
            if matching.len() == 1 {
                available.remove(&matching[0].0);
                mapping.insert(name, matching[0].0);
            }
        }
    }

    let my_ticket = parse_ticket(&sections[1][13..]);

    let mut product = 1;

    for (name, _) in &rules {
        if name.starts_with("departure") {
            product *= my_ticket[mapping[name]];
        }
    }

    println!("day16.part2.solution = {}", product);
}

fn parse_field_line(line: &str) -> (&str, Vec<(u64, u64)>) {
    let first_split = line.find(": ").unwrap();
    let second_split = line.find(" or ").unwrap();

    let mut ranges = vec![];
    ranges.push(parse_range(&line[first_split + 2..second_split]));
    ranges.push(parse_range(&line[second_split + 4..]));

    (&line[..first_split], ranges)
}

fn parse_range(range: &str) -> (u64, u64) {
    let split_idx = range.find("-").unwrap();
    (
        range[..split_idx].parse().unwrap(),
        range[split_idx + 1..].parse().unwrap(),
    )
}

fn parse_ticket(ticket: &str) -> Vec<u64> {
    ticket.split(",").map(|x| x.parse().unwrap()).collect()
}

fn valid_ticket(ticket: &Vec<u64>, ranges: &Vec<(u64, u64)>) -> bool {
    for field in ticket {
        if !valid_field(*field, ranges) {
            return false;
        }
    }
    return true;
}

fn valid_field(field: u64, ranges: &Vec<(u64, u64)>) -> bool {
    for (lower, upper) in ranges {
        if *lower <= field && field <= *upper {
            return true;
        }
    }
    return false;
}
