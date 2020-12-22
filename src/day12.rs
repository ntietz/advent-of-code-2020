use std::f64;
use std::fs;

pub fn run() {
    part1();
    part2();
}

pub fn part1() {
    let instructions: Vec<(String, i32)> = fs::read_to_string("inputs/day12.txt")
        .unwrap()
        .lines()
        .map(|x| (x[0..1].to_owned(), x[1..].parse().unwrap()))
        .collect();

    let mut angle: f64 = 0.0;
    let mut coords = (0, 0);

    for (action, value) in instructions {
        let translation = match &action[..] {
            "N" => (0, value),
            "S" => (0, -value),
            "E" => (value, 0),
            "W" => (-value, 0),
            "F" => (
                value * angle.cos().round() as i32,
                value * angle.sin().round() as i32,
            ),
            _ => (0, 0),
        };

        let rotation = match &action[..] {
            "L" => radians(value as f64),
            "R" => radians(-value as f64),
            _ => 0.0,
        };

        coords = (coords.0 + translation.0, coords.1 + translation.1);
        angle += rotation;
    }

    let distance = coords.0.abs() + coords.1.abs();

    println!("day12.part1.solution = {}", distance);
}

pub fn part2() {
    let instructions: Vec<(String, f64)> = fs::read_to_string("inputs/day12.txt")
        .unwrap()
        .lines()
        .map(|x| (x[0..1].to_owned(), x[1..].parse().unwrap()))
        .collect();

    let mut coords: (f64, f64) = (0.0, 0.0);
    let mut wp: (f64, f64) = (10.0, 1.0);

    for (action, value) in instructions {
        let wp_trans = match &action[..] {
            "N" => (0.0, value),
            "S" => (0.0, -value),
            "E" => (value, 0.0),
            "W" => (-value, 0.0),
            _ => (0.0, 0.0),
        };

        let wp_rot = match &action[..] {
            "L" => radians(value),
            "R" => radians(-value),
            _ => 0.0,
        };

        wp = (wp.0 + wp_trans.0, wp.1 + wp_trans.1);
        wp = (
            wp.0 * wp_rot.cos() - wp.1 * wp_rot.sin(),
            wp.0 * wp_rot.sin() + wp.1 * wp_rot.cos(),
        );

        if &action[..] == "F" {
            coords = (coords.0 + wp.0 * value, coords.1 + wp.1 * value);
        }
    }

    let distance = coords.0.abs() + coords.1.abs();

    println!("day12.part1.solution = {}", distance.round() as i32);
}

fn radians(degrees: f64) -> f64 {
    degrees * f64::consts::PI / 180.0
}
