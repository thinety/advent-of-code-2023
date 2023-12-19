mod parse;
use parse::parse;

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Default)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

pub fn part1(input: &str) -> u32 {
    let games = parse(input);

    games
        .iter()
        .filter_map(|game| {
            match game
                .rounds
                .iter()
                .all(|round| round.red <= 12 && round.green <= 13 && round.blue <= 14)
            {
                true => Some(game.id),
                false => None,
            }
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let games = parse(input);

    games
        .iter()
        .map(|game| {
            let acc = game
                .rounds
                .iter()
                .fold(Round::default(), |acc, round| Round {
                    red: acc.red.max(round.red),
                    green: acc.green.max(round.green),
                    blue: acc.blue.max(round.blue),
                });
            acc.red * acc.green * acc.blue
        })
        .sum()
}

crate::samples! {
    (part1_sample, part1, "sample.in", 8),
    (part1_puzzle, part1, "puzzle.in", 2331),
    (part2_sample, part2, "sample.in", 2286),
    (part2_puzzle, part2, "puzzle.in", 71585),
}
