struct Bus {
    id: usize,
    offset: usize,
}

fn main() {
    let mut lines = include_str!("../input").lines();

    let timestamp: usize = lines.next().unwrap().parse().unwrap();
    let busses: Vec<Bus> = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(offset, id)| id.parse().map(|id| Bus { id, offset }).ok())
        .collect();

    println!("Part 1: {}", part_1(timestamp, &busses));
    println!("Part 2: {}", part_2(&busses));
}

fn part_1(timestamp: usize, busses: &[Bus]) -> usize {
    let (id, waittime) = busses
        .iter()
        .map(|bus| (bus.id, (timestamp / bus.id + 1) * bus.id - timestamp))
        .min_by_key(|(_, waittime)| *waittime)
        .unwrap();

    id * waittime
}

fn part_2(busses: &[Bus]) -> usize {
    // This solution is based on https://topaz.github.io/paste/#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
    // My initial solution was a mess using the num crate, 128 bit integers and extended gcd.

    // If we have zero busses, they align at every step. This is what we start with.

    // Keeps track of the first time the busses align. For 0 busses, this is at step 0.
    let mut first = 0;

    // Steps after the first time where the busses align again. For 0 busses, this is every step (1).
    let mut period = 1;

    'bus: for &Bus { id, offset } in busses {
        for k in 0..id {
            let candidate = first + period * k;
            if (candidate + offset) % id == 0 {
                first = candidate;

                // set new period by finding lcm(period, id). Since all ids are prime this can be simplified to a product.
                period = period * id;
                continue 'bus;
            }
        }

        panic!("No solution");
    }

    first
}
