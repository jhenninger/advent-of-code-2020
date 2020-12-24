use std::{
    collections::{HashMap, HashSet},
    mem,
};

type Point = (i32, i32);
type Grid = HashSet<Point>;

fn main() {
    let input = include_str!("../input");

    let mut grid = create_grid(&input);
    println!("Part 1: {}", grid.len());

    game_of_life(&mut grid, 100);
    println!("Part 2: {}", grid.len());
}

fn create_grid(input: &str) -> Grid {
    let mut grid = HashSet::new();

    for line in input.lines() {
        let mut x = 0;
        let mut y = 0;

        let mut iter = line.chars();

        while let Some(c) = iter.next() {
            match c {
                'e' => x += 1,
                'w' => x -= 1,
                's' => match iter.next().unwrap() {
                    'e' => y += 1,
                    'w' => {
                        x -= 1;
                        y += 1;
                    }
                    _ => panic!(),
                },
                'n' => match iter.next().unwrap() {
                    'e' => {
                        x += 1;
                        y -= 1
                    }
                    'w' => y -= 1,
                    _ => panic!(),
                },
                _ => panic!(),
            };
        }
        let point = (x, y);

        if !grid.insert(point) {
            grid.remove(&point);
        }
    }

    grid
}

fn game_of_life(grid: &mut Grid, rounds: usize) {
    let mut next = HashSet::new();
    let mut neighbor_count = HashMap::new();

    for _ in 0..rounds {
        for &tile in grid.iter() {
            let count = [(1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1)]
                .iter()
                .filter(|d| {
                    let n = (tile.0 + d.0, tile.1 + d.1);
                    if grid.contains(&n) {
                        true
                    } else {
                        *neighbor_count.entry(n).or_insert(0) += 1;
                        false
                    }
                })
                .count();

            if count == 1 || count == 2 {
                next.insert(tile);
            }
        }

        for (n, c) in neighbor_count.drain() {
            if c == 2 {
                next.insert(n);
            }
        }

        mem::swap(&mut next, grid);
        next.clear();
    }
}
