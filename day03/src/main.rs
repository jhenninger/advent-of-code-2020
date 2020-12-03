fn main() {
    let lines: Vec<_> = include_str!("../input").lines().collect();

    println!("Part 1: {}", trees(&lines, 3, 1));

    let part_2: usize = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(x, y)| trees(&lines, x, y))
        .product();

    println!("Part 2: {}", part_2);
}

fn trees(lines: &[&str], dx: usize, dy: usize) -> usize {
    let mut x = 0;
    lines
        .iter()
        .step_by(dy)
        .filter(|row| {
            let idx = x;
            x = (x + dx) % row.len();
            row.as_bytes()[idx] == b'#'
        })
        .count()
}
