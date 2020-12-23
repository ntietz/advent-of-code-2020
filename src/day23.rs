use std::collections::VecDeque;

pub fn run() {
    //let input = "389125467";
    let input = "467528193";
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut cups: VecDeque<i32> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let max = *cups.iter().max().unwrap();

    for _ in 0..100 {
        next_round(&mut cups, max);
    }

    rotate_to_1(&mut cups);

    let solution = cups
        .iter()
        .skip(1)
        .map(|&c| c.to_string())
        .collect::<Vec<String>>()
        .join("");

    println!("day23.part1.solution = {}", solution);
}

fn part2(input: &str) {
    let mut cups: VecDeque<i32> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let first = cups.iter().max().unwrap() + 1;

    for cup in first..=1_000_000 {
        cups.push_back(cup);
    }

    let max = *cups.iter().max().unwrap();

    for _ in 0..100 {
        next_round(&mut cups, max);
    }


}

fn next_round(cups: &mut VecDeque<i32>, max: i32) {
    assert!(cups.len() > 4);

    let current = cups.pop_front().unwrap();
    let mut moving_cups: VecDeque<i32> = (0..3).map(|_| cups.pop_front().unwrap()).collect();

    let mut looking_for = current - 1;
    if looking_for <= 0 {
        looking_for += max;
    }
    while moving_cups.contains(&looking_for) {
        looking_for -= 1;
        if looking_for <= 0 {
            looking_for += max;
        }
    }

    let destination_idx = cups
        .iter()
        .position(|&c| c == looking_for)
        .unwrap();

    let insert_at = (destination_idx + 1) % (cups.len() + 1);
    cups.rotate_left(insert_at);
    while !moving_cups.is_empty() {
        cups.push_front(moving_cups.pop_back().unwrap());
        //cups.insert(insert_at, moving_cups.pop_back().unwrap());
    }
    cups.rotate_right(insert_at);

    cups.push_front(current);
    cups.rotate_left(1);
}

fn rotate_to_1(cups: &mut VecDeque<i32>) {
    let idx = cups.iter().position(|&c| c == 1).unwrap();
    cups.rotate_left(idx);
}
