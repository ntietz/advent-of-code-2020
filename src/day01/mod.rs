mod input;
use std::collections::HashSet;

pub fn run() {
    part1();
    part2();
}

pub fn part1_naive() {
    let entries = crate::day01::input::INPUT.to_vec();
    let target = 2020;

    for (index, first) in entries.iter().enumerate() {
        for second in entries[index + 1..].iter() {
            if first + second == target {
                println!("day01.part1 solution = {} (#naive)", first * second);
                break;
            }
        }
    }
}

pub fn part1() {
    let input = crate::day01::input::INPUT;
    let entries: HashSet<i32> = input.iter().copied().collect();
    let target = 2020;

    for entry in input.iter() {
        if entries.contains(&(target - entry)) {
            println!("day01.part1 solution = {}", entry * (target - entry));
            break;
        }
    }
}

pub fn part2_naive() {
    let entries = crate::day01::input::INPUT;
    let target = 2020;

    for i in 0..entries.len() - 2 {
        for j in i + 1..entries.len() - 1 {
            for k in j + 1..entries.len() {
                if entries[i] + entries[j] + entries[k] == target {
                    println!(
                        "day01.part2 solution = {} (#naive)",
                        entries[i] * entries[j] * entries[k]
                    );
                }
            }
        }
    }
}

pub fn part2() {
    let input = crate::day01::input::INPUT.to_vec();
    let entries: HashSet<i32> = input.iter().copied().collect();
    let target = 2020;

    'outer: for (index, x) in input.iter().enumerate() {
        for y in input[index + 1..].iter() {
            if entries.contains(&(target - x - y)) {
                println!("day01.part2.solution = {}", x * y * (target - x - y));
                break 'outer;
            }
        }
    }
}
