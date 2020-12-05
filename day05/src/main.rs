fn main() {
    let mut seats: Vec<_> = include_str!("../input")
        .lines()
        .map(|line| {
            let mut seat = 0;
            for (i, c) in line.as_bytes().iter().rev().enumerate() {
                if b"BR".contains(c) {
                    seat |= 1 << i;
                }
            }
            seat
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
