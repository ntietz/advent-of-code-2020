use std::collections::{HashSet, VecDeque};
use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day22.txt").unwrap();
    let sections = input.split("\n\n");

    let decks: Vec<VecDeque<u32>> = sections.map(load_deck).collect();

    let (_, score) = play(decks[0].clone(), decks[1].clone(), false);
    let (_, score_rec) = play(decks[0].clone(), decks[1].clone(), true);

    println!("day22.part1.solution = {}", score);
    println!("day22.part2.solution = {}", score_rec);
}

fn load_deck(input: &str) -> VecDeque<u32> {
    let mut lines = input.lines();
    lines.next(); // skip the header

    let mut cards = VecDeque::new();
    for line in lines {
        cards.push_back(line.parse().unwrap());
    }

    cards
}

fn compute_score(deck: &VecDeque<u32>) -> u32 {
    let mut score: u32 = 0;

    for (idx, card) in deck.iter().rev().enumerate() {
        score += (idx + 1) as u32 * card;
    }

    score
}

fn game_id(deck1: &VecDeque<u32>, deck2: &VecDeque<u32>) -> (u32, u32) {
    (compute_score(deck1), compute_score(deck2))
}

fn play(mut deck1: VecDeque<u32>, mut deck2: VecDeque<u32>, recursive: bool) -> (bool, u32) {
    let mut prev: HashSet<(u32, u32)> = HashSet::new();

    while !deck1.is_empty() && !deck2.is_empty() && !prev.contains(&game_id(&deck1, &deck2)) {
        prev.insert(game_id(&deck1, &deck2));
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();

        let p1_wins = match recursive {
            false => card1 > card2,
            true if deck1.len() < card1 as usize || deck2.len() < card2 as usize => card1 > card2,
            _ => {
                let subdeck1 = deck1.iter().take(card1 as usize).copied().collect();
                let subdeck2 = deck2.iter().take(card2 as usize).copied().collect();
                let (p1_wins, _) = play(subdeck1, subdeck2, recursive);
                p1_wins
            }
        };

        if p1_wins {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    let player1_wins = !deck1.is_empty() || prev.contains(&game_id(&deck1, &deck2));
    let winning_deck = if player1_wins { deck1 } else { deck2 };

    let score = compute_score(&winning_deck);

    (player1_wins, score)
}
