use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input").trim();
    let foods: Vec<_> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(" (contains ");
            let ingredients: Vec<_> = parts.next().unwrap().split_whitespace().collect();
            let allergens: Vec<_> = parts
                .next()
                .unwrap()
                .trim_end_matches(')')
                .split(", ")
                .collect();
            (ingredients, allergens)
        })
        .collect();

    let mut allergens_to_ingredients: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut safe_ingredients = HashSet::new();

    for (ingredients, allergens) in foods.iter() {
        safe_ingredients.extend(ingredients.iter().cloned());
        for allergen in allergens {
            allergens_to_ingredients
                .entry(allergen)
                .and_modify(|v| v.retain(|i| ingredients.contains(i)))
                .or_insert_with(|| ingredients.iter().cloned().collect());
        }
    }

    for ingredient in allergens_to_ingredients.values().flatten() {
        safe_ingredients.remove(ingredient);
    }

    let part_1: usize = foods
        .iter()
        .flat_map(|(ingredients, _)| ingredients.iter())
        .filter(|&ingredient| safe_ingredients.contains(ingredient))
        .count();

    println!("Part 1: {}", part_1);

    let mut ingredient_to_allergen = HashMap::new();

    while !allergens_to_ingredients.is_empty() {
        allergens_to_ingredients.retain(|allergen, ingredients| {
            if ingredients.len() == 1 {
                let ingredient = ingredients.iter().next().unwrap();
                ingredient_to_allergen.insert(*ingredient, *allergen);
                false
            } else {
                ingredients.retain(|i| !ingredient_to_allergen.contains_key(i));
                true
            }
        })
    }

    let mut pairs: Vec<_> = ingredient_to_allergen.into_iter().collect();
    pairs.sort_unstable_by_key(|&(_, allergen)| allergen);
    let part_2 = pairs
        .into_iter()
        .map(|(ingredient, _)| ingredient)
        .collect::<Vec<_>>()
        .join(",");

    println!("Part 2: {}", part_2);
}
