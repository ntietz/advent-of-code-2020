use std::cmp;
use std::collections::HashSet;
use std::fs;

pub fn part1() {
    let mut cube = ConwayCube::load("inputs/day17.txt");
    for _ in 0..6 {
        cube.cycle();
    }
    println!("day17.part1.solution = {}", cube.num_active());
}

pub fn part2() {
    let mut cube = ConwayCube::load("inputs/day17.txt");
    cube.enable_w();

    for _ in 0..6 {
        cube.cycle();
    }

    println!("day17.part2.solution = {}", cube.num_active());
}

type Coord = (i64, i64, i64, i64);

#[derive(Debug)]
struct ConwayCube {
    active: HashSet<Coord>,
    use_w: bool,
}

impl ConwayCube {
    pub fn load(filename: &str) -> ConwayCube {
        let input = fs::read_to_string(filename).unwrap();

        let mut active = HashSet::new();

        for (x, line) in input.lines().enumerate() {
            for (y, c) in line.chars().enumerate() {
                if c == '#' {
                    active.insert((x as i64, y as i64, 0, 0));
                }
            }
        }

        ConwayCube {
            active,
            use_w: false,
        }
    }

    pub fn enable_w(&mut self) {
        self.use_w = true;
    }

    pub fn num_active(&self) -> usize {
        self.active.len()
    }

    pub fn bounding_box(&self) -> (Coord, Coord) {
        let first_cube = *self.active.iter().next().unwrap_or(&(0, 0, 0, 0));
        let min = self.active.iter().fold(first_cube, |x, y| {
            (
                cmp::min(x.0, y.0 - 1),
                cmp::min(x.1, y.1 - 1),
                cmp::min(x.2, y.2 - 1),
                if self.use_w {
                    cmp::min(x.3, y.3 - 1)
                } else {
                    -1
                },
            )
        });
        let max = self.active.iter().fold(first_cube, |x, y| {
            (
                cmp::max(x.0, y.0 + 1),
                cmp::max(x.1, y.1 + 1),
                cmp::max(x.2, y.2 + 1),
                if self.use_w {
                    cmp::max(x.3, y.3 + 1)
                } else {
                    1
                },
            )
        });
        (min, max)
    }

    pub fn cycle(&mut self) {
        let mut next_active = HashSet::new();

        let ((minx, miny, minz, minw), (maxx, maxy, maxz, maxw)) = self.bounding_box();

        for x in minx..maxx + 1 {
            for y in miny..maxy + 1 {
                for z in minz..maxz + 1 {
                    for w in minw..maxw + 1 {
                        let coord = (x, y, z, w);
                        let neighbors = list_neighbors(coord, self.use_w);
                        let num_active_neighbors =
                            neighbors.iter().filter(|c| self.active.contains(c)).count();

                        if (self.active.contains(&coord) && num_active_neighbors == 2)
                            || num_active_neighbors == 3
                        {
                            next_active.insert(coord);
                        }
                    }
                }
            }
        }

        self.active = next_active;
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        let ((minx, miny, minz, minw), (maxx, maxy, maxz, maxw)) = self.bounding_box();

        for z in minz + 1..maxz {
            for w in minw + 1..maxw {
                println!("\nz = {}, w = {}", z, w);
                for x in minx + 1..maxx {
                    for y in miny + 1..maxy {
                        let coord = (x, y, z, w);
                        if self.active.contains(&coord) {
                            print!("#");
                        } else {
                            print!(".");
                        }
                    }
                    print!("\n");
                }
            }
        }
    }
}

fn list_neighbors(coord: Coord, use_w: bool) -> Vec<Coord> {
    let mut neighbors = vec![];
    let (x, y, z, w) = coord;

    let (min_w, max_w) = match use_w {
        true => (-1, 2),
        false => (0, 1),
    };

    for dx in -1..2 {
        for dy in -1..2 {
            for dz in -1..2 {
                for dw in min_w..max_w {
                    if dx != 0 || dy != 0 || dz != 0 || dw != 0 {
                        neighbors.push((x + dx, y + dy, z + dz, w + dw));
                    }
                }
            }
        }
    }

    neighbors
}
