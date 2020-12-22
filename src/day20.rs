use std::collections::HashMap;
use std::fs;

pub fn run() {
    part1();
    part2();
}

pub fn part1() {
    let input = fs::read_to_string("inputs/day20.txt").unwrap();
    let image = Image::load(&input);
    let corners = image.corners();

    let solution: u64 = corners.iter().map(|t| t.id).product();

    println!("day20.part1.solution = {}", solution);
}

pub fn part2() {
    let input = fs::read_to_string("inputs/day20.txt").unwrap();
    let mut image = Image::load(&input);

    let arranged = image.arranged();

    let mut full_image: Vec<Vec<char>> = vec![];

    for row in arranged {
        for subrow in 0..8 {
            let mut img_row = vec![];
            for col in &row {
                for c in &col.image[subrow] {
                    img_row.push(*c);
                }
            }
            full_image.push(img_row);
        }
    }

    let num_hash: usize = full_image
        .iter()
        .map(|row| row.iter().filter(|&c| *c == '#').count())
        .sum();
    let mut num_monsta = num_monsters(&full_image);

    while num_monsta == 0 {
        rot_90(&mut full_image);
        num_monsta = num_monsters(&full_image);

        if num_monsta == 0 {
            flip_h(&mut full_image);
            num_monsta = num_monsters(&full_image);
            flip_h(&mut full_image);
        }
    }

    let solution = num_hash - 15 * num_monsta;
    println!("day20.part2.solution = {}", solution);
}

#[allow(dead_code)]
fn print(image: &Vec<Vec<char>>) {
    for row in image {
        let mut s = String::from("");
        for c in row {
            s.push(*c);
        }
        println!("{}", s);
    }
    println!("");
}

fn num_monsters(image: &Vec<Vec<char>>) -> usize {
    let monster: Vec<Vec<char>> = vec![
        "                  # ".chars().collect(),
        "#    ##    ##    ###".chars().collect(),
        " #  #  #  #  #  #   ".chars().collect(),
    ];

    let mut monsters = 0;

    for row in 0..(image.len() - monster.len()) {
        for col in 0..(image[0].len() - monster[0].len()) {
            let mut matched = true;
            'mybrain: for (drow, monster_row) in monster.iter().enumerate() {
                for (dcol, c) in monster_row.iter().enumerate() {
                    match c {
                        '#' => {
                            if image[row + drow][col + dcol] != '#' {
                                matched = false;
                                break 'mybrain;
                            }
                        }
                        _ => {}
                    }
                }
            }

            if matched {
                monsters += 1;
            }
        }
    }

    monsters
}

fn rot_90(image: &mut Vec<Vec<char>>) {
    let mut rot_image = image.clone();

    for row in 0..image.len() {
        for col in 0..image.len() {
            rot_image[col][image.len() - 1 - row] = image[row][col];
        }
    }

    image.clear();
    image.append(&mut rot_image);
}

fn flip_h(image: &mut Vec<Vec<char>>) {
    for row in image {
        let len = row.len();
        for idx in 0..(len / 2) {
            row.swap(idx, len - idx - 1);
        }
    }
}

#[derive(Debug)]
struct Image {
    tiles: Vec<Tile>,
    size: usize,
}

impl Image {
    pub fn load(input: &str) -> Image {
        let tiles: Vec<_> = input.split("\n\n").map(Tile::load).collect();
        let size = (tiles.len() as f64).sqrt() as usize;
        Image { tiles, size }
    }

    pub fn arranged(&mut self) -> Vec<Vec<Tile>> {
        let border_counts = self.border_counts();

        let mut starting_corner = self.corners()[0].clone();

        // orient the starting corner correctly
        if border_counts[&starting_corner.borders[0]] > 1 {
            starting_corner.flip_v();
            assert!(border_counts[&starting_corner.borders[0]] == 1);
        }
        if border_counts[&starting_corner.borders[3]] > 1 {
            starting_corner.flip_h();
            assert!(border_counts[&starting_corner.borders[3]] == 1);
        }

        let mut remaining_tiles: Vec<_> = self
            .tiles
            .iter()
            .filter(|&t| t.id != starting_corner.id)
            .collect();

        let mut row = vec![starting_corner];

        for idx in 1..self.size {
            // find each next tile
            let sought = row[idx - 1].borders[1];
            let sought_alt = reversed_10(sought);
            let mut tile = (*remaining_tiles
                .iter()
                .find(|&t| t.borders.contains(&sought) || t.borders.contains(&sought_alt))
                .unwrap())
            .clone();
            remaining_tiles = remaining_tiles
                .iter()
                .filter(|&t| t.id != tile.id)
                .map(|&t| t)
                .collect();

            while border_counts[&tile.borders[0]] > 1 {
                tile.rot_90();
            }
            assert!(border_counts[&tile.borders[0]] == 1);
            if tile.borders[3] != sought {
                tile.flip_h();
                assert!(border_counts[&tile.borders[3]] == 2);
                assert!(tile.borders[3] == sought);
            }

            row.push(tile);
        }

        let mut arrangement = vec![row];

        for row_num in 1..self.size {
            let mut row = vec![];

            for col_num in 0..self.size {
                let sought = arrangement[row_num - 1][col_num].borders[2];
                let sought_alt = reversed_10(sought);

                let mut tile = (*remaining_tiles
                    .iter()
                    .find(|&t| t.borders.contains(&sought) || t.borders.contains(&sought_alt))
                    .unwrap())
                .clone();
                remaining_tiles = remaining_tiles
                    .iter()
                    .filter(|&t| t.id != tile.id)
                    .map(|&t| t)
                    .collect();

                while tile.borders[0] != sought && tile.borders[0] != sought_alt {
                    tile.rot_90();
                }

                if tile.borders[0] != sought {
                    tile.flip_h();
                }

                row.push(tile);
            }

            arrangement.push(row);
        }

        assert_eq!(remaining_tiles.len(), 0);

        arrangement
    }

    fn border_counts(&self) -> HashMap<u64, u64> {
        let mut counts: HashMap<u64, u64> = HashMap::new();
        for tile in &self.tiles {
            for border in &tile.borders {
                *counts.entry(*border).or_default() += 1;
                *counts.entry(reversed_10(*border)).or_default() += 1;
            }
        }

        counts
    }

    pub fn corners(&self) -> Vec<&Tile> {
        let border_counts = self.border_counts();
        let mut corners = vec![];

        for tile in &self.tiles {
            let mut shared = 0;
            for border in &tile.borders {
                if border_counts[border] > 1 || border_counts[&reversed_10(*border)] > 1 {
                    shared += 1;
                }
            }

            if shared == 2 {
                corners.push(tile);
            }
        }

        corners
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Tile {
    id: u64,
    borders: [u64; 4],
    image: [[char; 8]; 8],
}

impl Tile {
    fn load(input: &str) -> Tile {
        let mut lines = input.lines();
        let id_line = lines.next().unwrap();

        let id = id_line[5..id_line.len() - 1].parse().unwrap();
        let mut borders = [0, 0, 0, 0];

        let mut image = [[' '; 8]; 8];

        for (idx, line) in lines.enumerate() {
            let chars: Vec<_> = line.chars().collect();
            if idx == 0 {
                for bit in &chars {
                    borders[0] = (borders[0] << 1) | (*bit == '.') as u64;
                }
            }
            if idx == 9 {
                for bit in &chars {
                    borders[2] = (borders[2] << 1) | (*bit == '.') as u64;
                }
            }
            borders[1] = (borders[1] << 1) | (chars[9] == '.') as u64;
            borders[3] = (borders[3] << 1) | (chars[0] == '.') as u64;
            if 0 < idx && idx < 9 {
                for col in 0..8 {
                    image[idx - 1][col] = chars[col + 1];
                }
            }
        }

        Tile { id, borders, image }
    }

    fn flip_h(&mut self) {
        self.borders.swap(1, 3);
        self.borders[0] = reversed_10(self.borders[0]);
        self.borders[2] = reversed_10(self.borders[2]);

        for row in 0..8 {
            self.image[row].reverse();
        }
    }

    fn flip_v(&mut self) {
        self.borders.swap(0, 2);
        self.borders[1] = reversed_10(self.borders[1]);
        self.borders[3] = reversed_10(self.borders[3]);

        for col in 0..8 {
            for row in 0..4 {
                let tmp = self.image[row][col];
                self.image[row][col] = self.image[7 - row][col];
                self.image[7 - row][col] = tmp;
            }
        }
    }

    fn rot_90(&mut self) {
        let tmp = self.borders[3];
        self.borders[3] = self.borders[2];
        self.borders[2] = reversed_10(self.borders[1]);
        self.borders[1] = self.borders[0];
        self.borders[0] = reversed_10(tmp);

        let mut rot_image: [[char; 8]; 8] = [[' '; 8]; 8];

        for row in 0..8 {
            for col in 0..8 {
                rot_image[col][7 - row] = self.image[row][col];
            }
        }

        self.image = rot_image;
    }
}

fn reversed_10(border: u64) -> u64 {
    border.reverse_bits() >> (64 - 10)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn loads_tile_from_str() {
        let input = "Tile 1753:
..##.#.#.#
#...#.....
#......#..
#..##..#.#
#..##....#
#.........
#.#...##..
#....#..##
##.......#
#...######";

        let expected = Tile {
            id: 1753,
            borders: [810, 408, 448, 512],
        };

        assert_eq!(Tile::load(&input), expected);
    }
}
