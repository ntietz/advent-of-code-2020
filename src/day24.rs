use std::collections::HashSet;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day24.txt").unwrap();
    let coordinates = parse_instructions(&input);

    let mut flipped_tiles: HashSet<(i32, i32)> = HashSet::new();

    for coord in coordinates {
        if !flipped_tiles.contains(&coord) {
            flipped_tiles.insert(coord);
        } else {
            flipped_tiles.remove(&coord);
        }
    }

    println!("day24.part1.solution = {}", flipped_tiles.len());

    for _ in 0..100 {
        flipped_tiles = flip_next_day(&flipped_tiles);
    }

    println!("day24.part2.solution = {}", flipped_tiles.len());
}

fn flip_next_day(current: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut next = HashSet::new();

    for &tile in current {
        let neighbors = get_neighbors(tile);
        let up_neighbors: Vec<_> = neighbors.iter().filter(|c| current.contains(c)).collect();
        let down_neighbors: Vec<_> = neighbors.iter().filter(|c| !current.contains(c)).collect();

        if up_neighbors.len() == 1 || up_neighbors.len() == 2 {
            next.insert(tile);
        }

        for &neighbor in down_neighbors {
            let naynays = get_neighbors(neighbor);
            let up_naynays: Vec<_> = naynays.iter().filter(|c| current.contains(c)).collect();

            if up_naynays.len() == 2 {
                next.insert(neighbor);
            }
        }
    }

    next
}

fn get_neighbors((x, y): (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (x + 2, y),
        (x - 2, y),
        (x + 1, y + 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
        (x - 1, y - 1),
    ]
}

fn parse_instructions(instructions: &str) -> Vec<(i32, i32)> {
    instructions.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> (i32, i32) {
    let mut chars = line.chars();

    let mut x = 0;
    let mut y = 0;

    while let Some(c) = chars.next() {
        let done = match c {
            'e' => {
                x += 2;
                true
            }
            'w' => {
                x -= 2;
                true
            }
            'n' => {
                y += 1;
                false
            }
            's' => {
                y -= 1;
                false
            }
            _ => panic!("Found an unexpected input char, requires: n, e, s, w."),
        };
        if !done {
            match chars.next() {
                Some('e') => {
                    x += 1;
                }
                Some('w') => {
                    x -= 1;
                }
                _ => panic!("Found an unexpected input char, e/w must follow n/s."),
            }
        }
    }

    (x, y)
}
