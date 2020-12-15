use std::collections::HashMap;

fn main() {
    println!("Part 1: {}, Part 2: {}", nth(2020), nth(30000000));
}

fn nth(target: usize) -> usize {
    let mut start = [0, 6, 1, 7, 2, 19, 20].iter().cloned();
    let mut seen = HashMap::new();
    let mut previous = None;

    for turn in 0..target {
        let number = previous.map(|p| turn - seen.insert(p, turn).unwrap_or(turn));
        previous = start.next().or(number);
    }

    previous.unwrap()
}
