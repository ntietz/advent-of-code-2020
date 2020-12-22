use std::fs;

pub fn run() {
    part1();
    part2();
}

pub fn part1() {
    let raw_input = fs::read_to_string("inputs/day13.txt").unwrap();
    let mut input_lines = raw_input.lines();

    let start_ts: u32 = input_lines.next().unwrap().parse().unwrap();
    let bus_ids: Vec<u32> = input_lines
        .next()
        .unwrap()
        .split(',')
        .filter(|&s| s != "x")
        .map(|x| x.parse().unwrap())
        .collect();

    let earliest_id: u32 = *bus_ids.iter().min_by_key(|&x| x - (start_ts % x)).unwrap();
    let solution = earliest_id * (earliest_id - (start_ts % earliest_id));

    println!("day13.part1.solution = {}", solution);
}

pub fn part2() {
    let raw_input = fs::read_to_string("inputs/day13.txt").unwrap();
    let mut input_lines = raw_input.lines();

    input_lines.next(); // skip the first line
    let bus_ids: Vec<(u64, u64)> = input_lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(idx, x)| (idx as u64, x.parse().unwrap()))
        .collect();

    let (mut curr_ts, mut step) = bus_ids[0];

    for &(idx, ts) in &bus_ids[1..] {
        while (curr_ts + idx) % ts != 0 {
            curr_ts += step;
        }
        step *= ts;
    }

    println!("day13.part2.solution = {}", curr_ts);
}
