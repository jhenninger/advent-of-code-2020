fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", trees(input, 3, 1));

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let part_2: usize = slopes.iter().map(|&(x, y)| trees(input, x, y)).product();

    println!("Part 2: {}", part_2);
}

fn trees(input: &str, dx: usize, dy: usize) -> usize {
    let mut x = 0;
    input
        .lines()
        .step_by(dy)
        .filter(|row| {
            let idx = x;
            x = (x + dx) % row.len();
            row.as_bytes()[idx] == b'#'
        })
        .count()
}
