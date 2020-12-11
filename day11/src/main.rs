use std::mem;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Occupied,
    Empty,
    Floor,
}

type Point = (isize, isize);

#[rustfmt::skip]
const DIRS: [Point; 8] = [
    (-1, -1), ( 0, -1), (1, -1),
    (-1,  0),           (1,  0),
    (-1,  1), ( 0,  1), (1,  1),
];

#[derive(Clone)]
struct Grid(Vec<Vec<Tile>>);

impl Grid {
    fn get(&self, (x, y): Point) -> Option<Tile> {
        if x < 0 || y < 0 {
            return None;
        }

        self.0.get(y as usize)?.get(x as usize).cloned()
    }

    fn set(&mut self, (x, y): Point, tile: Tile) {
        if x < 0 || y < 0 {
            panic!("index < 0");
        }

        self.0[y as usize][x as usize] = tile;
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = (Point, Tile)> + 'a {
        self.0.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, tile)| ((x as isize, y as isize), *tile))
        })
    }
}

fn main() {
    let input = include_str!("../input");
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'L' => Tile::Empty,
                    '#' => Tile::Occupied,
                    '.' => Tile::Floor,
                    invalid => panic!("Invalid char: {}", invalid),
                })
                .collect()
        })
        .collect();
    let grid = Grid(grid);

    println!(
        "Part 1: {}\nPart 2: {}",
        run_until_equilibrium(&grid, 4, count_neighbors),
        run_until_equilibrium(&grid, 5, count_visible),
    );
}

fn run_until_equilibrium(
    initial: &Grid,
    threshold: usize,
    count_occupied: impl Fn(&Grid, Point) -> usize,
) -> usize {
    let mut current = initial.clone();
    let mut next = initial.clone();
    let mut changed = true;

    while changed {
        changed = false;
        for ((x, y), tile) in current.iter() {
            if tile == Tile::Floor {
                continue;
            }

            let occupied = count_occupied(&current, (x, y));

            let next_tile = match tile {
                Tile::Empty if occupied == 0 => Tile::Occupied,
                Tile::Occupied if occupied >= threshold => Tile::Empty,
                _ => tile,
            };

            changed = changed || tile != next_tile;
            next.set((x, y), next_tile);
        }

        mem::swap(&mut current, &mut next);
    }

    occupied(&current)
}

fn count_neighbors(grid: &Grid, (x, y): Point) -> usize {
    DIRS.iter()
        .filter(|(dx, dy)| grid.get((x + dx, y + dy)) == Some(Tile::Occupied))
        .count()
}

fn count_visible(grid: &Grid, (x, y): Point) -> usize {
    DIRS.iter()
        .filter(|(dx, dy)| {
            let mut x = x + dx;
            let mut y = y + dy;

            loop {
                match grid.get((x, y)) {
                    None | Some(Tile::Empty) => break false,
                    Some(Tile::Occupied) => break true,
                    _ => {
                        x = x + dx;
                        y = y + dy;
                    }
                }
            }
        })
        .count()
}

fn occupied(grid: &Grid) -> usize {
    grid.iter()
        .filter(|&(_, tile)| tile == Tile::Occupied)
        .count()
}
