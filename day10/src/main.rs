use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");

    let mut jolts: Vec<isize> = input.lines().map(|x| x.parse().unwrap()).collect();
    jolts.push(0);
    jolts.sort();
    jolts.push(jolts.last().unwrap() + 3);

    println!("Part 1: {}\nPart 2: {}", part_1(&jolts), part_2(&jolts));
}

fn part_1(jolts: &[isize]) -> isize {
    let steps = jolts
        .windows(2)
        .fold((0, 0), |acc, pair| match pair[1] - pair[0] {
            1 => (acc.0 + 1, acc.1),
            3 => (acc.0, acc.1 + 1),
            _ => acc,
        });

    steps.0 * steps.1
}

fn part_2(jolts: &[isize]) -> isize {
    let device = jolts.last().unwrap();
    let mut ways = HashMap::new();
    ways.insert(0, 1);

    for &jolt in jolts.iter().skip(1) {
        let count = (1..=3)
            .map(|d| ways.get(&(jolt - d)).cloned().unwrap_or_default())
            .sum();

        ways.insert(jolt, count);
    }

    *ways.get(device).unwrap()
}
