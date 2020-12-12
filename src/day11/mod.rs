use std::fs;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

#[derive(Debug)]
struct Layout {
    pub rows: i32,
    pub cols: i32,
    seats: Vec<Seat>,
}

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Layout {
    pub fn parse(raw: &str) -> Layout {
        let mut rows: i32 = 0;
        let mut seats: Vec<Seat> = Vec::new();
        for line in raw.lines() {
            for c in line.chars() {
                seats.push(match c {
                    'L' => Seat::Empty,
                    _ => Seat::Floor,
                })
            }
            rows += 1;
        }

        let cols = (seats.len() as i32) / rows;

        Layout { rows, cols, seats }
    }

    fn occupied_count(&self) -> u32 {
        self.seats.iter().filter(|&s| *s == Seat::Occupied).count() as u32
    }

    fn occupied_neighbors(&self, row: i32, col: i32) -> u32 {
        DIRECTIONS
            .iter()
            .map(|(r, c)| (row + r, col + c))
            .filter(|(r, c)| self.in_bounds(*r, *c))
            .filter(|(r, c)| self.seats[self.seat_idx(*r, *c)] == Seat::Occupied)
            .count() as u32
    }

    fn seat_idx(&self, row: i32, col: i32) -> usize {
        (row * self.cols + col) as usize
    }

    fn occupied_visible(&self, row: i32, col: i32) -> u32 {
        let mut count = 0;

        for (dr, dc) in &DIRECTIONS {
            let mut r = row + dr;
            let mut c = col + dc;

            while self.in_bounds(r, c) && self.seats[self.seat_idx(r, c)] == Seat::Floor {
                r += dr;
                c += dc;
            }

            if self.in_bounds(r, c) && self.seats[self.seat_idx(r, c)] == Seat::Occupied {
                count += 1;
            }
        }

        count
    }

    fn in_bounds(&self, row: i32, col: i32) -> bool {
        0 <= row && row < self.rows && 0 <= col && col < self.cols
    }

    pub fn step_adjacent(&mut self) -> bool {
        let mut next = self.seats.clone();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let idx = self.seat_idx(row, col);
                next[idx] = match (self.seats[idx], self.occupied_neighbors(row, col)) {
                    (Seat::Empty, 0) => Seat::Occupied,
                    (Seat::Occupied, x) if x >= 4 => Seat::Empty,
                    (s, _) => s,
                };
            }
        }

        let changed = next != self.seats;
        self.seats = next;
        changed
    }

    pub fn step_visible(&mut self) -> bool {
        let mut next = self.seats.clone();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let idx = self.seat_idx(row, col);
                next[idx] = match (self.seats[idx], self.occupied_visible(row, col)) {
                    (Seat::Empty, 0) => Seat::Occupied,
                    (Seat::Occupied, x) if x >= 5 => Seat::Empty,
                    (s, _) => s,
                };
            }
        }

        let changed = next != self.seats;
        self.seats = next;
        changed
    }
}

pub fn part1() {
    let mut layout = load_layout("inputs/day11.txt");
    //println!("{}", layout.occupied_count());
    while layout.step_adjacent() {
        //println!("{}", layout.occupied_count());
    }
    let solution = layout.occupied_count();
    println!("day11.part1.solution = {}", solution);
}

pub fn part2() {
    let mut layout = load_layout("inputs/day11.txt");
    //println!("{}", layout.occupied_count());
    while layout.step_visible() {
        //println!("{}", layout.occupied_count());
    }
    let solution = layout.occupied_count();
    println!("day11.part2.solution = {}", solution);
}

fn load_layout(filename: &str) -> Layout {
    Layout::parse(&fs::read_to_string(filename).unwrap())
}
