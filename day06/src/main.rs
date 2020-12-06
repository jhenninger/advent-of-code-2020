use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");

    let groups: Vec<(usize, HashMap<char, usize>)> = input
        .split("\n\n")
        .map(|group| {
            let mut map = HashMap::new();
            let mut count = 0;
            for person in group.lines() {
                count += 1;
                for yes in person.chars() {
                    *map.entry(yes).or_default() += 1
                }
            }
            (count, map)
        })
        .collect();

    let part_1: usize = groups.iter().map(|(_, a)| a.len()).sum();
    let part_2: usize = groups
        .iter()
        .map(|(c, a)| a.values().filter(|&y| y == c).count())
        .sum();

    println!("Part 1: {}\nPart 2: {}", part_1, part_2);
}
