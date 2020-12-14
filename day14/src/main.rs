use std::collections::HashMap;

enum Instr {
    Mask(Mask),
    Write { address: u64, value: u64 },
}

struct Mask {
    x: u64,
    bits: u64,
}

impl Mask {
    fn new(s: &str) -> Self {
        let mut x = 0;
        let mut bits = 0;
        for (i, c) in s.chars().rev().enumerate() {
            match c {
                '0' => (),
                '1' => bits |= 1 << i,
                'X' => x |= 1 << i,
                invalid => panic!("invalid char {}", invalid),
            };
        }
        Self { x, bits }
    }

    fn part_1(&self, mut value: u64) -> u64 {
        value |= self.bits; // set ones
        value &= self.x | self.bits; // set zeroes
        value
    }

    fn part_2(&self, mut address: u64) -> impl Iterator<Item = u64> {
        let floating = self.x;
        address &= !floating; // set all floating bits to zero
        address |= self.bits; // set ones

        let combinations = 2u64.pow(self.x.count_ones());

        (0..combinations).map(move |mut i| {
            let mut address = address;
            let mut target = 0; // position of the bit we are setting in address

            while i > 0 {
                target += (floating >> target).trailing_zeros();
                address |= (i & 1) << target;
                i >>= 1;
                target += 1;
            }

            address
        })
    }
}

fn main() {
    let prog: Vec<Instr> = include_str!("../input")
        .lines()
        .map(|line| {
            let mut iter = line.split(&['[', ']', ' '][..]);
            if iter.next().unwrap() == "mask" {
                let mask = iter.nth(1).unwrap();
                Instr::Mask(Mask::new(mask))
            } else {
                let address = iter.next().unwrap().parse().unwrap();
                let value = iter.nth(2).unwrap().parse().unwrap();
                Instr::Write { address, value }
            }
        })
        .collect();

    let part_1 = run(&prog, |address, value, mask, memory| {
        memory.insert(address, mask.part_1(value));
    });

    let part_2 = run(&prog, |address, value, mask, memory| {
        for address in mask.part_2(address) {
            memory.insert(address, value);
        }
    });

    println!("Part 1: {}\nPart 2: {}", part_1, part_2);
}

fn run(prog: &[Instr], write: impl Fn(u64, u64, &Mask, &mut HashMap<u64, u64>)) -> u64 {
    let mut memory = HashMap::new();
    let mut mask = None;

    for instr in prog {
        match instr {
            Instr::Mask(m) => mask = Some(m),
            Instr::Write { address, value } => write(*address, *value, &mask.unwrap(), &mut memory),
        }
    }

    memory.values().sum()
}
