use std::mem;
use Action::*;

enum Action {
    N,
    S,
    E,
    W,
    L,
    R,
    F,
}

struct Instr(Action, isize);

fn main() {
    let input = include_str!("../input");
    let instructions: Vec<Instr> = input
        .lines()
        .map(|line| {
            let number = line[1..].parse().unwrap();
            let action = match &line[..1] {
                "N" => N,
                "S" => S,
                "E" => E,
                "W" => W,
                "L" => L,
                "R" => R,
                "F" => F,
                invalid => panic!("invalid action {}", invalid),
            };

            Instr(action, number)
        })
        .collect();

    println!(
        "Part 1: {}\nPart 2: {}",
        part_1(&instructions),
        part_2(&instructions)
    );
}

fn part_1(instructions: &[Instr]) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 0;

    for Instr(action, value) in instructions {
        match action {
            N => y += value,
            S => y -= value,
            E => x += value,
            W => x -= value,
            L => rotate(-value / 90, &mut dx, &mut dy),
            R => rotate(value / 90, &mut dx, &mut dy),
            F => {
                x += dx * value;
                y += dy * value;
            }
        }
    }

    (x.abs() + y.abs()) as usize
}

fn part_2(instructions: &[Instr]) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut dx = 10;
    let mut dy = 1;

    for Instr(action, value) in instructions {
        match action {
            N => dy += value,
            S => dy -= value,
            E => dx += value,
            W => dx -= value,
            L => rotate(-value / 90, &mut dx, &mut dy),
            R => rotate(value / 90, &mut dx, &mut dy),
            F => {
                x += dx * value;
                y += dy * value;
            }
        }
    }

    (x.abs() + y.abs()) as usize
}

fn rotate(steps: isize, x: &mut isize, y: &mut isize) {
    for _ in 0..steps.rem_euclid(4) {
        mem::swap(x, y);
        *y = -*y;
    }
}
