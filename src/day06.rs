use std::fs;

pub fn run() {
    part1();
    part2();
}

pub fn part1() {
    let answer: u32 = fs::read_to_string("inputs/day6.txt")
        .unwrap()
        .split("\n\n")
        .map(|x| decode_group(x, or))
        .map(|x| x.count_ones())
        .sum();

    println!("day06.part1.solution = {}", answer);
}

pub fn part2() {
    let answer: u32 = fs::read_to_string("inputs/day6.txt")
        .unwrap()
        .split("\n\n")
        .map(|x| decode_group(x, and))
        .map(|x| x.count_ones())
        .sum();

    println!("day06.part2.solution = {}", answer);
}

fn or(x: u32, y: u32) -> u32 {
    x | y
}
fn and(x: u32, y: u32) -> u32 {
    x & y
}

fn decode_line(answers: &str) -> u32 {
    let mut d = 0;
    for b in answers.as_bytes() {
        d |= 1 << (b - b'a');
    }
    d
}

fn decode_group(answers: &str, combine: fn(u32, u32) -> u32) -> u32 {
    let mut lines = answers.lines();

    let mut d = match lines.next() {
        Some(line) => decode_line(line),
        None => 0,
    };

    for line in lines {
        d = combine(d, decode_line(line));
    }
    d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_line() {
        assert_eq!(decode_line("ace"), 21);
    }

    #[test]
    fn decodes_group() {
        assert_eq!(decode_group("ab\nce\nae", or), 23);
        assert_eq!(decode_group("ab\nace\nae", and), 1);
    }
}
