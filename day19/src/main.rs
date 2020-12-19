use std::collections::HashMap;

use regex::Regex;

enum Rule {
    Char(char),
    Rules(Vec<Vec<usize>>), // outer vec contains alternatives, inner vec contains sequences
}

type Rules = HashMap<usize, Rule>;

fn main() {
    let input = include_str!("../input");
    let mut iter = input.split("\n\n");

    let rules: Rules = iter
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let number = parts.next().unwrap().parse().unwrap();
            let rule = parts.next().unwrap();
            let rule = if rule.starts_with('"') {
                Rule::Char(rule.chars().nth(1).unwrap())
            } else {
                let alternatives = rule
                    .split(" | ")
                    .map(|alt| alt.split_whitespace().map(|n| n.parse().unwrap()).collect())
                    .collect();
                Rule::Rules(alternatives)
            };

            (number, rule)
        })
        .collect();

    let msgs: Vec<_> = iter.next().unwrap().lines().collect();

    println!(
        "Part 1: {}\nPart 2: {}",
        part_1(&rules, &msgs),
        part_2(&rules, &msgs)
    );
}

fn part_1(rules: &Rules, msgs: &[&str]) -> usize {
    let regex = build_regex(rules, false);
    msgs.iter().filter(|line| regex.is_match(line)).count()
}

fn part_2(rules: &Rules, msgs: &[&str]) -> usize {
    let regex = build_regex(rules, true);

    let mut r42 = String::new();
    build_regex_string(&rules, &mut r42, 42, false);
    let r42 = Regex::new(&r42).unwrap();

    let mut r31 = String::new();
    build_regex_string(&rules, &mut r31, 31, false);
    let r31 = Regex::new(&r31).unwrap();

    msgs.iter()
        .filter(|line| {
            regex
                .captures(line)
                .map(|captures| {
                    captures.iter().skip(1).flatten().all(|m| {
                        // m is a match of rule 11
                        let str = m.as_str();
                        for i in 0..str.len() {
                            let (left, right) = str.split_at(i);
                            // this is kinda buggy since the find_iter matches don't have to be consecutive, but it works for my input
                            if r42.find_iter(left).count() == r31.find_iter(right).count() {
                                return true;
                            }
                        }
                        false
                    })
                })
                .unwrap_or(false)
        })
        .count()
}

fn build_regex(rules: &Rules, part_2: bool) -> Regex {
    let mut s = String::from("^");
    build_regex_string(rules, &mut s, 0, part_2);
    s.push_str("$");
    s.parse().unwrap()
}

fn build_regex_string(rules: &Rules, s: &mut String, r: usize, part_2: bool) {
    if part_2 {
        if r == 8 {
            build_regex_string(rules, s, 42, true);
            s.push_str("+");
            return;
        }
        if r == 11 {
            s.push_str("(");
            build_regex_string(rules, s, 42, true);
            s.push_str("+");
            build_regex_string(rules, s, 31, true);
            s.push_str("+)");
            return;
        }
    }

    match &rules[&r] {
        Rule::Char(c) => s.push(*c),
        Rule::Rules(alternatives) => {
            s.push_str("(?:");
            let mut separator = "";
            for list in alternatives {
                s.push_str(separator);
                for rule in list {
                    build_regex_string(rules, s, *rule, part_2);
                }
                separator = "|";
            }
            s.push_str(")");
        }
    }
}
