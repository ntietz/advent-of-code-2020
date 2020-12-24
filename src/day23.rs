use std::collections::HashMap;

pub fn run() {
    let input = "467528193";
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let cups: Vec<i32> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    let max = *cups.iter().max().unwrap();
    let mut cups_succ: HashMap<i32, i32> = make_succ(&cups);

    run_rounds(&mut cups_succ, cups[0], max, 100);

    let mut solution = "".to_owned();
    let mut next = cups_succ[&1];
    while next != 1 {
        solution.push_str(&next.to_string());
        next = cups_succ[&next];
    }

    println!("day23.part1.solution = {}", solution);
}

fn part2(input: &str) {
    let mut cups: Vec<i32> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    for cup in cups.iter().max().unwrap() + 1..=1_000_000 {
        cups.push(cup);
    }

    let max = *cups.iter().max().unwrap();
    let mut cups_succ: HashMap<i32, i32> = make_succ(&cups);

    run_rounds(&mut cups_succ, cups[0], max, 10_000_000);

    let solution = cups_succ[&1] as u64 * cups_succ[&cups_succ[&1]] as u64;
    println!("day23.part2.solution = {}", solution);
}

fn run_rounds(succ: &mut HashMap<i32, i32>, first: i32, max: i32, num_rounds: usize) {
    let mut current = first;

    for _ in 0..num_rounds {
        let mut pick_up = [0; 3];
        pick_up[0] = succ[&current];
        pick_up[1] = succ[&pick_up[0]];
        pick_up[2] = succ[&pick_up[1]];

        let target = looking_for(current, max, &pick_up);

        succ.insert(current, succ[&pick_up[2]]);

        let tmp = succ[&target];
        succ.insert(target, pick_up[0]);
        succ.insert(pick_up[2], tmp);

        current = succ[&current];
    }
}

fn make_succ(cups: &[i32]) -> HashMap<i32, i32> {
    let mut succ: HashMap<i32, i32> = cups.windows(2).map(|s| (s[0], s[1])).collect();
    succ.insert(cups[cups.len() - 1], cups[0]);

    succ
}

/// This function is very inelegant. It feels like there's a much better way to do this,
/// and brain isn't braining right now. And let's be honest, I don't want to write the
/// tests right nwo that a trickier solution would require.
fn looking_for(current: i32, max: i32, skip: &[i32]) -> i32 {
    for offset in 1..=4 {
        let target = (((current - offset) + max - 1) % max) + 1;
        if skip[0] != target && skip[1] != target && skip[2] != target {
            return target;
        }
    }
    panic!("Did not find target.");
}
