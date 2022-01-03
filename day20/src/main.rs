use std::{fmt::Debug, mem, writeln};

#[derive(Clone)]
struct Tile {
    id: usize,
    squares: Vec<Vec<bool>>,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tile {}:", self.id)?;

        for row in self.squares.iter() {
            for cell in row {
                write!(f, "{}", if *cell { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Tile {
    // vllt neighbor methode, die auch den anderen tile orientieren kann?
    fn flip(&mut self) {
        self.squares.reverse();
    }

    fn rotate(&mut self) {
        let mut new = vec![vec![false; self.squares.len()]; self.squares.len()];

        for (y, row) in self.squares.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                new[x][row.len() - y - 1] = *tile;
            }
        }
        self.squares = new;
    }

    fn borders(&self, flip: bool) -> Vec<Vec<bool>> {
        let mut result = Vec::new();
        let mut clone = self.clone();

        for _ in 0..4 {
            let row = clone.squares[0].clone();
            result.push(row);
            clone.rotate();
        }

        if flip {
            clone.flip();
            for _ in 0..4 {
                let row = clone.squares[0].clone();
                result.push(row);
                clone.rotate();
            }
        }
        result
    }
}

#[test]
fn tile_test() {
    let mut t = Tile {
        id: 0,
        squares: vec![
            vec![true, true, false],
            vec![false, false, false],
            vec![false, false, false],
        ],
    };

    let e = Tile {
        id: 0,
        squares: vec![
            vec![false, false, true],
            vec![false, false, true],
            vec![false, false, false],
        ],
    };

    t.rotate();

    assert_eq!(t.squares, e.squares);

    let e = Tile {
        id: 0,
        squares: vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![false, true, true],
        ],
    };
    t.rotate();
    assert_eq!(t.squares, e.squares);

    let mut t = Tile {
        id: 0,
        squares: vec![
            vec![true, true, false],
            vec![false, false, false],
            vec![false, false, false],
        ],
    };

    let e = Tile {
        id: 0,
        squares: vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![true, true, false],
        ],
    };

    t.flip();
    assert_eq!(t.squares, e.squares);
}

fn main() {
    let input = include_str!("../input").trim();
    let monster = "
                  # 
#    ##    ##    ###
 #  #  #  #  #  #   
"
    .trim_matches('\n');

    let mut tiles: Vec<_> = input
        .split("\n\n")
        .map(|tile| {
            let mut rows = tile.lines();
            let id = rows.next().unwrap()[5..9].parse().unwrap();

            let squares = rows
                .map(|row| row.chars().map(|c| c == '#').collect())
                .collect();

            Tile { id, squares }
        })
        .collect();
    let size = (tiles.len() as f64).sqrt() as usize;

    let mut corners = Vec::new();

    for tile in tiles.iter() {
        let borders = tile.borders(true);

        let mut count = 0;
        for other in &tiles {
            if other.id == tile.id {
                continue;
            }
            let other_borders = other.borders(false);
            for border in borders.iter() {
                if other_borders.contains(border) {
                    count += 1;
                    break;
                }
            }
        }

        if count == 2 {
            corners.push(tile.clone());
        }
    }

    let part1 = corners.iter().map(|t| t.id).product::<usize>();
    dbg!(part1);

    let mut top_left = corners[0].clone();

    tiles.retain(|t| t.id != top_left.id);

    // orient tile so it is actually top left

    'outer: for i in 0..8 {
        let bottom = top_left.squares.last().unwrap();
        let right = {
            let mut clone = top_left.clone();
            clone.rotate();
            clone.squares.pop().unwrap()
        };

        let mut found_right = false;
        let mut found_bottom = false;

        for tile in tiles.iter() {
            let borders = tile.borders(false);
            if !found_bottom && borders.contains(bottom) {
                found_bottom = true;
            }
            if !found_right && borders.contains(&right) {
                found_right = true;
            }
            if found_bottom && found_right {
                break 'outer;
            }
        }

        if i == 3 {
            top_left.flip()
        } else {
            top_left.rotate();
        }
    }

    let mut x = 0;
    let mut y = 0;

    let mut grid = vec![
        vec![
            Tile {
                id: 0,
                squares: Vec::new()
            };
            size
        ];
        size
    ];

    grid[0][0] = top_left;

    while !tiles.is_empty() {
        let current = &grid[y][x];
        let bottom = current.squares.last().unwrap();
        let tile = tiles.iter().find_map(|t| {
            let mut t = (*t).clone();
            for i in 0..8 {
                if t.squares.first().unwrap() == bottom {
                    return Some(t);
                }

                if i == 3 {
                    t.flip()
                } else {
                    t.rotate()
                }
            }
            None
        });

        if let Some(tile) = tile {
            y += 1;
            tiles.retain(|x| x.id != tile.id);
            grid[y][x] = tile;
        } else {
            y = 0;
            let mut current = grid[y][x].clone();
            x += 1;
            current.rotate();
            let right = current.squares.last().unwrap();

            let tile = tiles.iter().find_map(|t| {
                let mut t = (*t).clone();

                for i in 0..8 {
                    let top = t.squares.first().unwrap().clone();
                    if top == *right {
                        return Some(t);
                    }

                    if i == 4 {
                        t.flip()
                    } else {
                        t.rotate()
                    }
                }

                None
            });

            if let Some(mut tile) = tile {
                tile.rotate();
                tile.rotate();
                tile.rotate();
                tiles.retain(|x| x.id != tile.id);
                grid[y][x] = tile;
            } else {
                panic!();
            }
        }
    }

    // make a huge tile

    for row in grid.iter_mut() {
        for tile in row.iter_mut() {
            tile.squares.pop();
            tile.squares.remove(0);

            for row in tile.squares.iter_mut() {
                row.pop();
                row.remove(0);
            }
        }
    }

    let tilesize = grid[0][0].squares.len();
    let hugesize = size * tilesize;

    let mut huge = vec![vec![false; hugesize]; hugesize];

    for y in 0..hugesize {
        for x in 0..hugesize {
            huge[y][x] = grid[y / tilesize][x / tilesize].squares[y % tilesize][x % tilesize];
        }
    }

    let mut huge = Tile {
        id: 0,
        squares: huge,
    };

    let monster: Vec<Vec<_>> = monster
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let monster_height = monster.len();
    let monster_width = monster[0].len();

    let mut monsters = 0;
    for i in 0..8 {
        for y in 0..hugesize - monster_height {
            'x: for x in 0..hugesize - monster_width {
                for (my, row) in monster.iter().enumerate() {
                    for (mx, tile) in row.iter().enumerate() {
                        if *tile && !huge.squares[y + my][x + mx] {
                            continue 'x;
                        }
                    }
                }

                monsters += 1;
            }
        }

        if monsters > 0 {
            break;
        }

        if i == 3 {
            huge.flip();
        } else {
            huge.rotate();
        }
    }

    let monster_chars = monster
        .iter()
        .flat_map(|row| row.iter().filter(|x| **x))
        .count();
    let grid_chars = huge
        .squares
        .iter()
        .flat_map(|row| row.iter().filter(|x| **x))
        .count();

    dbg!(grid_chars - monster_chars * monsters);
}
