use std::ops::RangeInclusive;

struct Field {
    name: String,
    ranges: Vec<RangeInclusive<u32>>,
    positions: u32,
}

impl Field {
    fn new(s: &str) -> Self {
        let mut parts = s.split(": ");
        let name = parts.next().unwrap().to_string();
        let ranges = parts
            .next()
            .unwrap()
            .split(" or ")
            .map(|range| {
                let mut bounds = range.split('-').map(|bound| bound.parse().unwrap());
                bounds.next().unwrap()..=bounds.next().unwrap()
            })
            .collect();

        Self {
            name,
            ranges,
            positions: u32::MAX,
        }
    }

    fn contains(&self, item: &u32) -> bool {
        self.ranges.iter().any(|range| range.contains(item))
    }
}

fn parse_ticket(s: &str) -> Vec<u32> {
    s.split(',').map(|n| n.parse().unwrap()).collect()
}

fn main() {
    let input = include_str!("../input");

    let mut parts = input.split("\n\n");
    let fields = parts.next().unwrap();
    let ticket = parts.next().unwrap();
    let nearby = parts.next().unwrap();

    let mut fields: Vec<_> = fields.lines().map(Field::new).collect();
    let ticket = ticket.lines().nth(1).map(parse_ticket).unwrap();

    let mut part_1 = 0;

    let valid: Vec<_> = nearby
        .lines()
        .skip(1)
        .map(parse_ticket)
        .filter(|ticket| {
            let errors: u32 = ticket
                .iter()
                .filter(|value| !fields.iter().any(|field| field.contains(value)))
                .sum();
            part_1 += errors;
            errors == 0
        })
        .collect();

    println!("Part 1: {}", part_1);

    for field in fields.iter_mut() {
        for pos in 0..ticket.len() {
            for ticket in valid.iter() {
                if !field.contains(&ticket[pos]) {
                    field.positions &= !(1 << pos);
                    break;
                }
            }
        }
    }

    fields.sort_by_key(|f| f.positions.count_ones());

    let mut mask = 0;
    for field in fields.iter_mut() {
        field.positions &= !mask;
        mask |= field.positions;
    }

    let part_2: u64 = fields
        .iter()
        .filter(|f| f.name.starts_with("departure"))
        .map(|f| ticket[f.positions.trailing_zeros() as usize] as u64)
        .product();

    println!("Part 2: {}", part_2);
}
