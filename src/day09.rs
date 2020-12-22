use std::collections::HashMap;
use std::fs;

pub fn run() {
    part1();
    part2();
}

pub fn part1() {
    let nums: Vec<u64> = fs::read_to_string("inputs/day9.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    let window_size = 26;

    let answer = first_invalid(&nums, window_size).unwrap();
    println!("day09.part1.solution = {}", answer);
}

pub fn part2() {
    let nums: Vec<u64> = fs::read_to_string("inputs/day9.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    let window_size = 26;

    let weakness = first_invalid(&nums, window_size).unwrap();
    let range_which_sums = inchworm_find(&nums, weakness).unwrap();
    let min = range_which_sums.iter().min().unwrap();
    let max = range_which_sums.iter().max().unwrap();

    println!("day09.part2.solution = {:#?}", min + max);
}

fn valid_num(preamble: &[u64], x: u64) -> bool {
    let mut partials: HashMap<u64, u64> = HashMap::new();
    for y in preamble {
        if *y < x {
            let count = partials.entry(x - y).or_insert(0);
            *count += 1;
        }
    }
    for y in preamble {
        if partials.contains_key(&y) {
            if partials[&y] > 1 || (y << 1 != x) {
                return true;
            }
        }
    }

    false
}

fn first_invalid(nums: &[u64], window_size: usize) -> Option<u64> {
    for window in nums.windows(window_size) {
        let preamble = &window[..window_size - 1];
        let num = window[window_size - 1];
        if !valid_num(preamble, num) {
            return Some(num);
        }
    }
    None
}

fn inchworm_find(nums: &[u64], target: u64) -> Option<&[u64]> {
    let mut sum = 0;
    let mut start = 0;
    let mut stop = 0;

    while stop < nums.len() && sum != target {
        // expand
        while stop < nums.len() && sum < target {
            sum += nums[stop];
            stop += 1;
        }

        // contract
        if start < stop && sum > target {
            sum -= nums[start];
            start += 1;
        }
    }

    match sum {
        x if x == target => Some(&nums[start..stop]),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn checks_validity() {
        assert!(valid_num(&[35, 20, 15, 25, 47], 40));
        assert!(valid_num(&[20, 15, 25, 47, 40], 62));
        assert!(!valid_num(&[95, 102, 117, 150, 182], 127));
        assert!(valid_num(&[25, 25], 50));
        assert!(!valid_num(&[25], 50));
    }
}
