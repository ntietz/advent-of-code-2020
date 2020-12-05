use std::fs;

#[derive(Debug)]
struct Terrain {
    pub map: Vec<Vec<char>>,
    pub rows: usize,
    cols: usize,
}

impl Terrain {
    pub fn load(filename: &str) -> Terrain {
        let map: Vec<_> = fs::read_to_string(filename)
            .unwrap()
            .lines()
            .map(|x| String::from(x).chars().collect::<Vec<char>>())
            .collect();
        let rows = map.len();
        let cols = map[0].len();

        Terrain { map, rows, cols }
    }

    pub fn contains_huggable_tree(&self, row: usize, col: usize) -> bool {
        return self.map[row][col % self.cols] == '#';
    }
}

fn how_many_trees_to_hug(terrain: &Terrain, d_row: usize, d_col: usize) -> usize {
    let mut row = 0;
    let mut col = 0;
    let mut trees_hugged = 0;

    while row < terrain.rows {
        trees_hugged += terrain.contains_huggable_tree(row, col) as usize;
        row += d_row;
        col += d_col;
    }

    return trees_hugged;
}

pub fn part1() {
    let filename = "inputs/day3.txt";
    let terrain = Terrain::load(filename);

    let solution = how_many_trees_to_hug(&terrain, 1, 3);

    println!("day3.part1.solution = {}", solution);
}

pub fn part2() {
    let filename = "inputs/day3.txt";
    let terrain = Terrain::load(filename);

    let slopes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    let solution: usize = slopes
        .into_iter()
        .map(|(d_row, d_col)| how_many_trees_to_hug(&terrain, d_row, d_col))
        .product();
    println!("day3.part2.solution = {}", solution);
}
