use std::collections::{BTreeMap, HashSet};
use std::fs;

pub fn run() {
    parts1and2();
}

pub fn parts1and2() {
    let input = fs::read_to_string("inputs/day21.txt").unwrap();

    let labels: Vec<IngredientsList> = input.lines().map(IngredientsList::parse).collect();

    let all_allergens: HashSet<String> = labels
        .iter()
        .map(|l| &l.allergens)
        .fold(HashSet::new(), |acc, next| {
            acc.union(&next).map(|s| s.to_owned()).collect()
        });
    let all_ingredients: HashSet<String> = labels
        .iter()
        .map(|l| &l.ingredients)
        .fold(HashSet::new(), |acc, next| {
            acc.union(&next).map(|s| s.to_owned()).collect()
        });

    let mut possible: HashSet<String> = HashSet::new();

    for allergen in &all_allergens {
        let possibilities: HashSet<String> = labels
            .iter()
            .filter(|l| l.allergens.contains(allergen))
            .fold(all_ingredients.clone(), |acc, next| {
                acc.intersection(&next.ingredients)
                    .map(|s| s.to_owned())
                    .collect()
            });
        possible = possible
            .union(&possibilities)
            .map(|s| s.to_owned())
            .collect();
    }

    let num_safe_appearances: u32 = labels
        .iter()
        .map(|l| {
            l.ingredients
                .iter()
                .filter(|&i| !possible.contains(i))
                .count() as u32
        })
        .sum();
    println!("day21.part1.solution = {}", num_safe_appearances);

    let mut allergen_identities: BTreeMap<String, String> = BTreeMap::new();

    while !possible.is_empty() {
        for allergen in &all_allergens {
            let possibilities: HashSet<String> = labels
                .iter()
                .filter(|l| l.allergens.contains(allergen))
                .fold(possible.clone(), |acc, next| {
                    acc.intersection(&next.ingredients)
                        .map(|s| s.to_owned())
                        .collect()
                });
            if possibilities.len() == 1 {
                let ing = possibilities.iter().next().unwrap();
                allergen_identities.insert(allergen.to_owned(), ing.to_owned());
                possible.remove(ing);
            }
        }
    }

    let canonical = (allergen_identities
        .values()
        .map(|s| s.to_owned())
        .collect::<Vec<_>>()[..])
        .join(",");
    println!("day21.part2.solution = {}", canonical);
}

#[derive(Clone, Debug)]
struct IngredientsList {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl IngredientsList {
    pub fn parse(line: &str) -> IngredientsList {
        let paren = line.find('(').unwrap();

        let ingredients = line[..paren - 1]
            .split_whitespace()
            .map(String::from)
            .collect();
        let allergens = line[paren + 10..line.len() - 1]
            .split(", ")
            .map(String::from)
            .collect();

        IngredientsList {
            ingredients,
            allergens,
        }
    }
}
