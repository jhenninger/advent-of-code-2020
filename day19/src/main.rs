use std::collections::HashMap;

use pcre2::bytes::Regex;

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

    let msgs: Vec<_> = iter.next().unwrap().lines().map(|l| l.as_bytes()).collect();

    println!(
        "Part 1: {}\nPart 2: {}",
        count(&rules, msgs.as_slice(), false),
        count(&rules, msgs.as_slice(), true)
    );
}

fn count(rules: &Rules, msgs: &[&[u8]], part_2: bool) -> usize {
    let regex = build_regex(rules, part_2);
    msgs.iter()
        .filter(|line| regex.is_match(line).unwrap_or(false))
        .count()
}

fn build_regex(rules: &Rules, part_2: bool) -> Regex {
    let mut s = String::from("^");
    build_regex_string(rules, &mut s, 0, part_2);
    s.push_str("$");
    Regex::new(&s).unwrap()
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
            s.push_str("(?1)?");
            build_regex_string(rules, s, 31, true);
            s.push_str(")");
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
