use std::collections::HashSet;
use std::collections::{hash_map::Entry, HashMap};
use std::io::{self, BufRead};

fn load_entries() -> Vec<(Vec<String>, Vec<String>)> {
    let mut result = Vec::new();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let ingredients_and_allergens: Vec<&str> = line.splitn(2, " (contains ").collect();
        let ingredients = ingredients_and_allergens[0]
            .split(' ')
            .map(|s| s.to_owned())
            .collect();
        let allergens = ingredients_and_allergens[1]
            .trim_end_matches(')')
            .split(", ")
            .map(|s| s.to_owned())
            .collect();
        result.push((ingredients, allergens));
    }
    result
}

fn get_ingredients_with_allergens(
    entries: &[(Vec<String>, Vec<String>)],
) -> HashMap<String, String> {
    let mut allergens_candidates: HashMap<String, HashSet<String>> = HashMap::new();
    // For each allergen, construct a list of candidate of ingredients.
    for (ingredients, allergens) in entries.iter() {
        let ingredients: HashSet<String> = ingredients.iter().cloned().collect();
        for allergen in allergens {
            match allergens_candidates.entry(allergen.to_owned()) {
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() = entry
                        .get()
                        .intersection(&ingredients)
                        .cloned()
                        .collect::<HashSet<String>>();
                }
                Entry::Vacant(entry) => {
                    entry.insert(ingredients.clone());
                }
            }
        }
    }
    let mut ingredients_with_allergens = HashMap::new();
    while !allergens_candidates.is_empty() {
        allergens_candidates.retain(|allergen, ingredients| {
            let ingredients = ingredients
                .iter()
                .filter(|ingredient| !ingredients_with_allergens.contains_key(*ingredient))
                .cloned()
                .collect::<HashSet<String>>();
            if ingredients.len() == 1 {
                let ingredient = ingredients.iter().next().unwrap();
                ingredients_with_allergens.insert(ingredient.clone(), allergen.clone());
            }
            ingredients.len() > 1
        });
    }
    ingredients_with_allergens
}

fn solve1(
    entries: &[(Vec<String>, Vec<String>)],
    ingredients_with_allergens: &HashMap<String, String>,
) {
    let count: usize = entries
        .iter()
        .flat_map(|entry| entry.0.iter())
        .filter(|ingredient| !ingredients_with_allergens.contains_key(*ingredient))
        .count();
    println!("Result: {}", count);
}

fn solve2(ingredients_with_allergens: &HashMap<String, String>) {
    let mut ingredients_with_allergens: Vec<(&String, &String)> =
        ingredients_with_allergens.iter().collect();
    ingredients_with_allergens.sort_by_key(|entry| entry.1);
    let result = ingredients_with_allergens
        .iter()
        .map(|entry| entry.0.clone())
        .collect::<Vec<String>>()
        .join(",");
    println!("Result: {}", result);
}

fn main() {
    let entries = load_entries();
    let ingredients_with_allergens = get_ingredients_with_allergens(&entries);
    solve1(&entries, &ingredients_with_allergens);
    solve2(&ingredients_with_allergens);
}
