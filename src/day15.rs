use std::collections::HashMap;

pub fn run() {
    part1();
    part2();
}

pub fn part1() {
    let mut spoken: Vec<usize> = vec![20, 9, 11, 0, 1, 2];
    let mut prev = spoken[spoken.len() - 1];
    let num_rounds = 2020;

    for turn in spoken.len()..num_rounds {
        let reused = spoken[..spoken.len() - 1].contains(&prev);

        if !reused {
            spoken.push(0);
            prev = 0;
        } else {
            let pos = spoken[..spoken.len() - 1]
                .iter()
                .rposition(|x| *x == prev)
                .unwrap();
            spoken.push(turn - pos - 1);
            prev = turn - pos - 1;
        }
    }

    let solution = spoken[num_rounds - 1];
    println!("day15.part1.solution = {}", solution);
}

pub fn part2() {
    let mut spoken: Vec<usize> = vec![20, 9, 11, 0, 1, 2];

    let mut prev_indices: HashMap<usize, Vec<usize>> = HashMap::new();
    for (index, each) in spoken[..spoken.len() - 1].iter().enumerate() {
        prev_indices.insert(*each, vec![index]);
    }
    let mut prev = spoken[spoken.len() - 1];
    let num_rounds = 30000000;

    for turn in spoken.len()..num_rounds {
        let prev_locs = prev_indices.entry(prev).or_default();
        if prev_locs.len() <= 1 {
            prev_locs.push(turn - 1);
        } else {
            prev_locs[0] = prev_locs[1];
            prev_locs[1] = turn - 1;
        }

        prev = if prev_locs.len() > 1 {
            turn - prev_locs[prev_locs.len() - 2] - 1
        } else {
            0
        };

        spoken.push(prev);
    }

    let solution = spoken[num_rounds - 1];
    println!("day15.part2.solution = {}", solution);
}
