fn main() {
    let million = 1_000_000;
    let mut map = vec![0; million+1];

    let input = "467528193".trim();

    let mut prev = million;
    for current in input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .chain(10..=million)
    {
        map[prev] = current;
        prev = current;
    }

    let mut current = map[million];

    for _ in 0..10 * million {
        let pick_1 = map[current];
        let pick_2 = map[pick_1];
        let pick_3 = map[pick_2];

        let mut dest = current - 1;

        while dest == 0 || dest == pick_1 || dest == pick_2 || dest == pick_3 {
            if dest == 0 {
                dest = million;
            } else {
                dest -= 1;
            }
        }
        let after_dest = map[dest];
        map[dest] = pick_1;
        let after_pick = map[pick_3];
        map[pick_3] = after_dest;
        map[current] = after_pick;
        current = after_pick;
    }

    let a = map[1];
    let b = map[a];
    assert_eq!(a*b, 264692662390);
}
