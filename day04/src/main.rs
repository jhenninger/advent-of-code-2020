use std::collections::HashMap;

fn main() {
    let fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let part_1: Vec<_> = include_str!("../input")
        .split("\n\n")
        .map(|pp| {
            pp.split_whitespace()
                .filter_map(|pair| {
                    let mut iter = pair.splitn(2, ':');
                    Some((iter.next()?, iter.next()?))
                })
                .collect::<HashMap<&str, &str>>()
        })
        .filter(|pp| fields.iter().all(|field| pp.contains_key(field)))
        .collect();

    println!("Part 1: {}", part_1.len());

    let part_2 = part_1.iter().filter(|pp| check(pp)).count();

    println!("Part 2: {}", part_2);
}

fn check(passport: &HashMap<&str, &str>) -> bool {
    passport.iter().all(|(&key, &value)| match key {
        "byr" => value
            .parse::<usize>()
            .map(|n| (1920..=2002).contains(&n))
            .unwrap_or(false),
        "iyr" => value
            .parse::<usize>()
            .map(|n| (2010..=2020).contains(&n))
            .unwrap_or(false),
        "eyr" => value
            .parse::<usize>()
            .map(|n| (2020..=2030).contains(&n))
            .unwrap_or(false),
        "hgt" => {
            match (
                value[0..value.len() - 2].parse::<usize>(),
                &value[value.len() - 2..],
            ) {
                (Ok(x), "cm") => (150..=193).contains(&x),
                (Ok(x), "in") => (59..=76).contains(&x),
                _ => false,
            }
        }
        "hcl" => value
            .strip_prefix("#")
            .map(|rest| rest.len() == 6 && rest.chars().all(|c| c.is_digit(16)))
            .unwrap_or(false),
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
        "pid" => value.len() == 9 && value.chars().all(|c| c.is_digit(10)),
        "cid" => true,
        _ => false,
    })
}
