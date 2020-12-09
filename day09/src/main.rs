use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    let numbers: Vec<isize> = input.lines().map(|x| x.parse().unwrap()).collect();

    let part_1 = part_1(&numbers).unwrap();
    println!("Part 1: {}", part_1);

    let part_2 = part_2(&numbers, part_1).unwrap();
    println!("Part 2: {}", part_2);
}

fn part_1(numbers: &[isize]) -> Option<isize> {
    let preamble = 25;
    let mut set = HashSet::new();

    for (i, &n) in numbers.iter().enumerate() {
        if i >= preamble {
            if !set.iter().any(|e| set.contains(&(n - e))) {
                return Some(n);
            }
            set.remove(&numbers[i - preamble]);
        }
        set.insert(n);
    }
    None
}

fn part_2(numbers: &[isize], needle: isize) -> Option<isize> {
    let mut sum = 0;
    let mut lo = 0;
    let mut hi = 0;

    while hi < numbers.len() {
        sum += numbers[hi];
        hi += 1;

        while sum > needle {
            sum -= numbers[lo];
            lo += 1;
        }

        if sum == needle {
            let iter = numbers[lo..hi].iter();
            let min = iter.clone().min();
            let max = iter.max();
            return Some(min? + max?);
        }
    }
    None
}
