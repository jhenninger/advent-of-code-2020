use std::{collections::HashSet, str::FromStr};

struct Handheld<'a> {
    ip: isize,
    acc: isize,
    prog: &'a [Instr],
}

struct Instr {
    op: Op,
    arg: isize,
}

impl FromStr for Instr {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let op = words.next().ok_or("missing operation")?.parse()?;
        let arg = words
            .next()
            .ok_or("missing arg")?
            .parse()
            .or(Err("invalid arg"))?;

        Ok(Instr { op, arg })
    }
}

enum Op {
    Nop,
    Acc,
    Jmp,
}

impl FromStr for Op {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = match s {
            "nop" => Op::Nop,
            "acc" => Op::Acc,
            "jmp" => Op::Jmp,
            _ => return Err("invalid opcode"),
        };

        Ok(op)
    }
}

impl<'a> Handheld<'a> {
    fn new(prog: &'a [Instr]) -> Self {
        Handheld {
            ip: 0,
            acc: 0,
            prog,
        }
    }

    fn step(&mut self) -> bool {
        if let Some(Instr { op, arg }) = self.prog.get(self.ip as usize) {
            match op {
                Op::Nop => (),
                Op::Acc => self.acc += arg,
                Op::Jmp => self.ip += arg - 1,
            }

            self.ip += 1;
            false
        } else {
            true
        }
    }
}

fn main() {
    let prog: Vec<Instr> = include_str!("../input")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    println!(
        "Part 1: {}\nPart 2: {}",
        run(&prog).unwrap_err(),
        part_2(prog).unwrap()
    );
}

fn run(prog: &[Instr]) -> Result<isize, isize> {
    let mut handheld = Handheld::new(prog);
    let mut seen = HashSet::new();
    while seen.insert(handheld.ip) {
        if handheld.step() {
            return Ok(handheld.acc);
        }
    }
    Err(handheld.acc)
}

fn part_2(mut prog: Vec<Instr>) -> Option<isize> {
    for i in 0..prog.len() {
        if toggle(&mut prog[i].op) {
            let result = run(&prog);
            if result.is_ok() {
                return result.ok();
            }
            toggle(&mut prog[i].op);
        }
    }
    None
}

fn toggle(op: &mut Op) -> bool {
    *op = match op {
        Op::Acc => return false,
        Op::Jmp => Op::Nop,
        Op::Nop => Op::Jmp,
    };
    true
}
