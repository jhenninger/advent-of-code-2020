use std::collections::{HashMap, HashSet};

type Rules<'a> = HashMap<&'a str, Vec<(usize, &'a str)>>;

fn main() {
    let input = include_str!("../input");
    let bag = "shiny gold";

    let rules: Rules = input
        .lines()
        .map(|line| {
            let sep = " bags contain ";
            let sep_pos = line.find(sep).unwrap();
            let bag = &line[..sep_pos];
            let contents = line[sep_pos + sep.len()..]
                .split(", ")
                .filter_map(|content| {
                    let space_pos = content.find(' ').unwrap();
                    let count = content[..space_pos].parse().ok()?;
                    let bag_pos = content.find(" bag").unwrap();
                    let color = &content[space_pos + 1..bag_pos];
                    Some((count, color))
                })
                .collect();
            (bag, contents)
        })
        .collect();

    let mut colors = HashSet::new();
    containing_colors(&rules, &mut colors, bag);

    println!(
        "Part 1: {}\nPart 2: {}",
        colors.len(),
        bag_contents(&rules, bag)
    );
}

fn containing_colors<'a>(rules: &Rules<'a>, colors: &mut HashSet<&'a str>, bag: &str) {
    for (outer, contents) in rules {
        for (_, color) in contents {
            if *color == bag {
                if colors.insert(outer) {
                    containing_colors(rules, colors, outer);
                }
                break;
            }
        }
    }
}

fn bag_contents(rules: &Rules, bag: &str) -> usize {
    if let Some(contents) = rules.get(bag) {
        contents
            .iter()
            .map(|(count, bag)| count + count * bag_contents(rules, bag))
            .sum()
    } else {
        0
    }
}
