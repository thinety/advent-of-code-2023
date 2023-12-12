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
    (part1_sample, part1, "sample.in", "13"),
    (part1_puzzle, part1, "puzzle.in", "23673"),
    (part2_sample, part2, "sample.in", "30"),
    (part2_puzzle, part2, "puzzle.in", "12263631"),
}
