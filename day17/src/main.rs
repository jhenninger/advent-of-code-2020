use std::{
    collections::{HashMap, HashSet},
    mem,
};

macro_rules! conway {
    ($dimen:expr, $init:expr) => {{
        type Point = [isize; $dimen];

        fn neighbors(p: &Point) -> impl Iterator<Item = Point> + '_ {
            let combinations = 3isize.pow($dimen as u32);
            (0..combinations)
                .filter(move |&n| n != combinations / 2)
                .map(move |mut n| {
                    let mut arr = [0; $dimen];
                    for (i, e) in p.iter().enumerate() {
                        arr[i] = e + n % 3 - 1;
                        n /= 3;
                    }
                    arr
                })
        }

        let mut current: HashSet<Point> = $init
            .iter()
            .map(|&(x, y)| {
                let mut arr = [0; $dimen];
                arr[0] = x;
                arr[1] = y;
                arr
            })
            .collect();

        let mut next = HashSet::new();

        for _ in 0..6 {
            let mut inactive_neighbors: HashMap<Point, usize> = HashMap::new();

            for c in current.iter() {
                let neighbors = neighbors(c)
                    .filter(|neighbor| {
                        let active = current.contains(neighbor);
                        if !active {
                            *inactive_neighbors.entry(*neighbor).or_default() += 1;
                        }

                        active
                    })
                    .count();

                if neighbors == 2 || neighbors == 3 {
                    next.insert(c.clone());
                }
            }

            for (c, neighbors) in inactive_neighbors.drain() {
                if neighbors == 3 {
                    next.insert(c);
                }
            }

            mem::swap(&mut current, &mut next);
            next.clear();
        }

        current.len()
    }};
}

fn main() {
    let initial: Vec<(isize, isize)> = include_str!("../input")
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .collect();

    println!(
        "Part 1: {}\nPart 2: {}",
        conway!(3, initial),
        conway!(4, initial)
    )
}
