use std::time::Instant;

fn main() {
    let start = Instant::now();

    let input = include_str!("../input");
    let entries: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    println!(
        "Part 1: {:?}\nPart 2: {:?}\nTook {}µs",
        part_1(&entries),
        part_2(&entries),
        start.elapsed().as_micros()
    );
}

fn part_1(entries: &[u64]) -> Option<u64> {
    for (i, a) in entries[..entries.len() - 1].iter().enumerate() {
        for b in entries[i + 1..].iter() {
            if a + b == 2020 {
                return Some(a * b);
            }
        }
    }

    None
}

fn part_2(entries: &[u64]) -> Option<u64> {
    for (i, a) in entries[..entries.len() - 2].iter().enumerate() {
        for (j, b) in entries[i + 1..entries.len() - 1].iter().enumerate() {
            for c in entries[i + j + 2..].iter() {
                if a + b + c == 2020 {
                    return Some(a * b * c);
                }
            }
        }
    }

    None
}
