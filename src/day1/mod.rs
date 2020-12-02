mod input;
use std::collections::HashSet;

pub fn part1_naive() {
    let entries = crate::day1::input::INPUT;
    let target = 2020;

    for i in 0..entries.len() - 1 {
        for j in i + 1..entries.len() {
            if entries[i] + entries[j] == target {
                println!("day1.part1 solution = {} (#naive)", entries[i] * entries[j]);
            }
        }
    }
}

pub fn part1() {
    let input = crate::day1::input::INPUT;
    let entries: HashSet<i32> = input.iter().cloned().collect();
    let target = 2020;

    for entry in input.iter() {
        if entries.contains(&(target - entry)) {
            println!("day1.part1 solution = {}", entry * (target - entry));
            break;
        }
    }
}

pub fn part2_naive() {
    let entries = crate::day1::input::INPUT;
    let target = 2020;

    for i in 0..entries.len() - 2 {
        for j in i + 1..entries.len() - 1 {
            for k in j + 1..entries.len() {
                if entries[i] + entries[j] + entries[k] == target {
                    println!(
                        "day1.part2 solution = {} (#naive)",
                        entries[i] * entries[j] * entries[k]
                    );
                }
            }
        }
    }
}

pub fn part2() {
    let input = crate::day1::input::EXAMPLE_INPUT;
    let entries: HashSet<i32> = input.iter().cloned().collect();
    let target = 2020;
    let mut partial_entries: HashSet<i32> = HashSet::new();

    let mut sorted_entries = input.to_vec();
    sorted_entries.sort();

    let mut left = 0;
    let mut right = input.len() - 1;

    for entry in input.iter() {
        partial_entries.insert(target - entry);
    }
    println!("{:#?}", partial_entries);

    while left < right {
        let partial = sorted_entries[left] + sorted_entries[right];
        let candidate = target - partial;
        println!(
            "{} / {} -- {}, {}: {}, {}",
            partial, candidate, left, right, sorted_entries[left], sorted_entries[right]
        );

        if candidate < sorted_entries[left] {
            right -= 1;
        } else if !entries.contains(&candidate) {
            left += 1;
        } else {
            println!("NOOO");
            break;
        }
    }
    println!("{} {}", left, right);
    println!("{:#?}", sorted_entries);
}
