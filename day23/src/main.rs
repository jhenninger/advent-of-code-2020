fn main() {
    let input = "467528193".trim();
    let mut initial = vec![0; input.len()];

    let mut prev = input.len() - 1;
    for c in input.chars() {
        let current = c.to_digit(10).unwrap() as usize - 1;
        initial[prev] = current;
        prev = current;

    dbg!(&initial);

    dbg!(part_1(&initial) );

}

fn play(cups: &mut [usize], rounds: usize) {
    let max = cups.len() - 1;
    let mut current = cups[max];

    for _ in 0..rounds {
        let pick_1 = cups[current];
        let pick_2 = cups[pick_1];
        let pick_3 = cups[pick_2];

        let mut dest = current;
        if dest == 0 {
            dest = max;
        } else {
            dest -= 1;
        }

        while dest == pick_1 || dest == pick_2 || dest == pick_3 {
            if dest == 0 {
                dest = max;
            } else {
                dest -= 1;
            }
        }
        let after_dest = cups[dest];
        cups[dest] = pick_1;
        let after_pick = cups[pick_3];
        cups[pick_3] = after_dest;
        cups[current] = after_pick;
        current = after_pick;
    }
}

fn part_1(initial: &[usize]) -> String {
    let mut cups = initial.to_vec();
    play(&mut cups, 100);
    let mut result = String::new();
    let mut current = cups[1];
    for _ in 0..8 {
        result.push(std::char::from_digit(current as u32 + 1, 10).unwrap());
        current = cups[current];
    }
    result
}

fn part_2(initial: &[usize]) -> usize {
    3
}
