use std::fs;

type SeatId = u32;

pub fn part1() {
    let seat_ids: Vec<_> = fs::read_to_string("inputs/day5.txt")
        .unwrap()
        .lines()
        .map(|x| decode_pass(x.trim()))
        .collect();
    let highest = seat_ids.iter().max().unwrap();

    println!("day5.part1.solution = {}", highest);
}

pub fn part2() {
    let mut seat_ids: Vec<_> = fs::read_to_string("inputs/day5.txt")
        .unwrap()
        .lines()
        .map(|x| decode_pass(x.trim()))
        .collect();
    seat_ids.sort();

    for window in seat_ids.windows(2) {
        if window[0] + 1 != window[1] {
            println!("day5.part2.solution = {}", window[0] + 1);
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
