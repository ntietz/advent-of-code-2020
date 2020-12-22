use std::collections::HashMap;
use std::fs;

pub fn run() {
    part1();
    part2();
}

pub fn part1() {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask = (0, 0);

    let input = fs::read_to_string("inputs/day14.txt").unwrap();

    for line in input.lines() {
        if line.starts_with("mask") {
            mask = load_bitmask(&line[7..]);
        } else {
            let (addr, val) = load_mem_cmd(line);
            *mem.entry(addr).or_default() = apply_mask(val, mask);
        }
    }

    let solution: u64 = mem.values().sum();

    println!("day14.part1.solution = {}", solution);
}

fn load_bitmask(s: &str) -> (u64, u64) {
    let mut and_mask: u64 = 0;
    let mut or_mask: u64 = 0;

    for c in s.chars() {
        and_mask <<= 1;
        or_mask <<= 1;

        match c {
            'X' => {
                and_mask |= 1;
            }
            '1' => {
                or_mask |= 1;
            }
            _ => {}
        }
    }

    (and_mask, or_mask)
}

fn load_mem_cmd(s: &str) -> (u64, u64) {
    let parts: Vec<&str> = s.split(|c| "[] ".contains(c)).collect();
    (parts[1].parse().unwrap(), parts[4].parse().unwrap())
}

fn apply_mask(val: u64, mask: (u64, u64)) -> u64 {
    (val & mask.0) | mask.1
}

pub fn part2() {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut masks: Vec<(u64, u64)> = Vec::new();

    let input = fs::read_to_string("inputs/day14.txt").unwrap();
    for line in input.lines() {
        if line.starts_with("mask") {
            masks = possible_masks(&line[7..]);
        } else {
            let (addr, val) = load_mem_cmd(line);
            for &mask in &masks {
                let addr = apply_mask(addr, mask);
                *mem.entry(addr).or_default() = val;
            }
        }
    }

    let solution: u64 = mem.values().sum();

    println!("day14.part2.solution = {}", solution);
}

fn possible_masks(s: &str) -> Vec<(u64, u64)> {
    let mut masks = vec![(0, 0)];

    for c in s.chars() {
        let mut new_masks = vec![];

        for mask in &masks {
            let (and, or) = mask;

            match c {
                '0' => {
                    new_masks.push(((and << 1) | 1, or << 1));
                }
                '1' => {
                    new_masks.push(((and << 1) | 1, (or << 1) | 1));
                }
                'X' => {
                    new_masks.push((and << 1, or << 1));
                    new_masks.push((and << 1, (or << 1) | 1));
                }
                _ => {}
            }
        }

        masks = new_masks;
    }

    masks
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn applies_mask() {
        let mask = load_bitmask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(apply_mask(11, mask), 73);
        assert_eq!(apply_mask(101, mask), 101);
        assert_eq!(apply_mask(0, mask), 64);
    }

    #[test]
    fn loads_mem_cmd() {
        assert_eq!(load_mem_cmd("mem[81] = 10"), (81, 10));
    }

    #[test]
    fn generates_multiple_masks() {
        let bitmask = "00000000000000000000000000000000X0XX";
        let masks = possible_masks(bitmask);
        assert_eq!(masks.len(), 8);
    }

    #[test]
    fn generates_correct_memory_addresses() {
        let address = 26;
        let bitmask = "00000000000000000000000000000000X0XX";

        let expected_addresses = vec![16, 17, 18, 19, 24, 25, 26, 27];

        let addresses: Vec<_> = possible_masks(bitmask)
            .iter()
            .map(|&mask| apply_mask(address, mask))
            .collect();

        assert_eq!(expected_addresses, addresses);
    }
}
