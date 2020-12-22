use std::cmp;
use std::collections::HashMap;
use std::fs;

pub fn run() {
    part1();
    part2();
}

pub fn part1() {
    let mut joltages: Vec<u64> = fs::read_to_string("inputs/day10.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    joltages.push(0);
    joltages.sort_unstable();

    let mut diffs: HashMap<u64, u64> = HashMap::new();

    for window in joltages.windows(2) {
        let count = diffs.entry(window[1] - window[0]).or_default();
        *count += 1;
    }

    *diffs.entry(3).or_default() += 1;

    let solution = diffs[&1] * diffs[&3];

    println!("day10.part1.solution = {}", solution);
}

pub fn part2() {
    let mut joltages: Vec<u64> = fs::read_to_string("inputs/day10.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    joltages.push(0);
    joltages.sort_unstable();
    joltages.push(3 + joltages[joltages.len() - 1]);

    let mut contiguous = 1;
    let mut arrangements = 1;

    for window in joltages.windows(2) {
        if window[1] - window[0] == 1 {
            contiguous += 1;
        } else {
            if contiguous >= 3 {
                let all: u64 = 1 << (contiguous - 2);
                let disallowed: u64 = (1 << cmp::max(0, contiguous - 4)) - 1;
                arrangements *= all - disallowed;
            }
            contiguous = 1;
        }
    }

    println!("day10.part2.solution = {}", arrangements);
}
