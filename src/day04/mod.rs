mod parse;
use parse::parse;

use std::collections::HashSet;

struct Card {
    numbers: HashSet<u32>,
    winning_numbers: HashSet<u32>,
}

pub fn part1(input: &str) -> u32 {
    let cards = parse(input);
    cards
        .iter()
        .map(|card| {
            let numbers = card.numbers.intersection(&card.winning_numbers).count();
            (1 << numbers) >> 1
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let cards = parse(input);

    let mut v = vec![0; cards.len()];
    let mut s = 1;
    let mut total = 0;

    for (i, card) in cards.iter().enumerate() {
        total += s;

        let n = card.numbers.intersection(&card.winning_numbers).count();

        v[i + n] += s;
        s = 2 * s - v[i];
    }

    total as u32
}

crate::samples! {
    (part1, part1_sample, "sample.in", "sample.out1"),
    (part1, part1_puzzle, "puzzle.in", "puzzle.out1"),
    (part2, part2_sample, "sample.in", "sample.out2"),
    (part2, part2_puzzle, "puzzle.in", "puzzle.out2"),
}
