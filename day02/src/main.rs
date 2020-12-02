use regex::Regex;

struct Entry<'a> {
    a: usize,
    b: usize,
    letter: u8,
    word: &'a [u8],
}

fn main() {
    let input = include_str!("../input");
    let regex = Regex::new(r"(?m)^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    let entries: Vec<_> = regex
        .captures_iter(input)
        .map(|captures| Entry {
            a: captures.get(1).unwrap().as_str().parse().unwrap(),
            b: captures.get(2).unwrap().as_str().parse().unwrap(),
            letter: captures.get(3).unwrap().as_str().as_bytes()[0],
            word: captures.get(4).unwrap().as_str().as_bytes(),
        })
        .collect();

    println!("Part 1: {}\nPart 2: {}", part_1(&entries), part_2(&entries));
}

fn part_1(entries: &[Entry]) -> usize {
    entries
        .iter()
        .filter(|entry| {
            let count = entry.word.iter().filter(|&&c| c == entry.letter).count();
            (entry.a..=entry.b).contains(&count)
        })
        .count()
}

fn part_2(entries: &[Entry]) -> usize {
    entries
        .iter()
        .filter(|&entry| {
            let &Entry { a, b, word, letter } = entry;
            (word[a - 1] == letter) ^ (word[b - 1] == letter)
        })
        .count()
}
