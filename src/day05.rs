use std::fs;

pub fn run() {
    part1();
    part2();
}

type SeatId = u32;

pub fn part1() {
    let seat_ids: Vec<_> = fs::read_to_string("inputs/day5.txt")
        .unwrap()
        .lines()
        .map(|x| decode_pass(x.trim()))
        .collect();
    let highest = seat_ids.iter().max().unwrap();

    println!("day05.part1.solution = {}", highest);
}

pub fn part2() {
    let mut max: u32 = 0;
    let mut min: u32 = !max;
    let mut sum: u32 = 0;

    for line in fs::read_to_string("inputs/day5.txt").unwrap().lines() {
        let seat_id = decode_pass(&line.trim());
        if seat_id < min {
            min = seat_id;
        } else if seat_id > max {
            max = seat_id;
        }
        sum += seat_id;
    }
    let expected_sum = (max - min + 1) * (max + min) / 2;
    let my_seat_id = expected_sum - sum;
    println!("day05.part2.solution = {}", my_seat_id);
}

pub fn part2_take1() {
    let mut seat_ids: Vec<_> = fs::read_to_string("inputs/day5.txt")
        .unwrap()
        .lines()
        .map(|x| decode_pass(x.trim()))
        .collect();
    seat_ids.sort_unstable();

    for window in seat_ids.windows(2) {
        if window[0] + 1 != window[1] {
            println!("day05.part2.solution = {}", window[0] + 1);
            break;
        }
    }
}

fn decode_pass(pass: &str) -> SeatId {
    let mut seat_id = 0;

    for c in pass.chars() {
        seat_id <<= 1;
        seat_id |= (c == 'B' || c == 'R') as u32;
    }

    seat_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_passes() {
        assert_eq!(decode_pass("BFFFBBFRRR"), 567);
    }
}
