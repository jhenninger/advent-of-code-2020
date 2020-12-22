use std::{
    collections::{hash_map::DefaultHasher, HashSet, VecDeque},
    hash::{Hash, Hasher},
};

fn main() {
    let input = include_str!("../input");

    let mut iter = input.split("\n\n").map(|side| {
        let stack: VecDeque<u8> = side
            .lines()
            .skip(1)
            .map(|line| line.parse().unwrap())
            .collect();
        stack
    });

    let p1 = iter.next().unwrap();
    let p2 = iter.next().unwrap();

    println!(
        "Part 1: {}, Part 2: {}",
        play(p1.clone(), p2.clone(), normal),
        play(p1, p2, recursive)
    );
}

fn play<F>(mut p1: VecDeque<u8>, mut p2: VecDeque<u8>, variant: F) -> usize
where
    F: FnOnce(&mut VecDeque<u8>, &mut VecDeque<u8>) -> bool,
{
    let winner = if variant(&mut p1, &mut p2) { p1 } else { p2 };
    winner
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i + 1) * *c as usize)
        .sum()
}

fn normal(p1: &mut VecDeque<u8>, p2: &mut VecDeque<u8>) -> bool {
    while !p1.is_empty() && !p2.is_empty() {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }

    p2.is_empty()
}

fn hash(p1: &VecDeque<u8>, p2: &VecDeque<u8>) -> u64 {
    let mut hasher = DefaultHasher::new();
    p1.hash(&mut hasher);
    p2.hash(&mut hasher);
    hasher.finish()
}

fn recursive(p1: &mut VecDeque<u8>, p2: &mut VecDeque<u8>) -> bool {
    let mut cache: HashSet<u64> = HashSet::new();

    while !p1.is_empty() && !p2.is_empty() {
        if !cache.insert(hash(p1, p2)) {
            return true;
        }

        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        let p1_wins = if p1.len() >= c1 as usize && p2.len() >= c2 as usize {
            recursive(
                &mut p1.iter().cloned().take(c1 as usize).collect(),
                &mut p2.iter().cloned().take(c2 as usize).collect(),
            )
        } else {
            c1 > c2
        };

        if p1_wins {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }

    p2.is_empty()
}
