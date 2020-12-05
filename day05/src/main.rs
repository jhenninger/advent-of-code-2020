fn main() {
    let mut seats: Vec<_> = include_str!("../input")
        .lines()
        .map(|line| {
            let mut f = 0;
            let mut b = 127;
            let mut l = 0;
            let mut r = 7;

            for c in line.chars() {
                match c {
                    'F' => b = (f + b) / 2,
                    'B' => f = ((f + b) as f64 / 2.0).ceil() as u64,
                    'L' => r = (l + r) / 2,
                    'R' => l = ((l + r) as f64 / 2.0).ceil() as u64,
                    wtf => panic!("wtf {}", wtf),
                }
            }

            f * 8 + l
        })
        .collect();

    seats.sort_unstable();

    println!("Part 1: {}", seats.last().unwrap());

    for pair in seats.windows(2) {
        if pair[0] + 2 == pair[1] {
            println!("Part 2: {}", pair[0] + 1);
            break;
        }
    }
}
