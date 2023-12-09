use std::collections::HashSet;
use std::io::Read;

use anyhow::Result;

struct Card {
    numbers: HashSet<u32>,
    winning_numbers: HashSet<u32>,
}

fn parse(input: &str) -> Vec<Card> {
    use nom::{
        bytes::complete::tag,
        character::complete::{digit1, space1},
        combinator::{map, map_res},
        multi::separated_list1,
        sequence::{delimited, pair, preceded, separated_pair},
        IResult,
    };

    fn number(input: &str) -> IResult<&str, u32> {
        map_res(digit1, str::parse)(input)
    }
    fn numbers(input: &str) -> IResult<&str, HashSet<u32>> {
        map(separated_list1(space1, number), |numbers| {
            numbers.into_iter().collect()
        })(input)
    }
    fn card(input: &str) -> IResult<&str, Card> {
        map(
            preceded(
                delimited(pair(tag("Card"), space1), number, pair(tag(":"), space1)),
                separated_pair(numbers, delimited(space1, tag("|"), space1), numbers),
            ),
            |(winning_numbers, numbers)| Card {
                numbers,
                winning_numbers,
            },
        )(input)
    }
    fn cards(input: &str) -> IResult<&str, Vec<Card>> {
        separated_list1(tag("\n"), card)(input)
    }

    let (input, cards) = cards(input.trim_end()).unwrap();
    assert!(input.is_empty());
    cards
}

fn part1(input: &str) -> u32 {
    let cards = parse(input);
    cards
        .iter()
        .map(|card| {
            let numbers = card.numbers.intersection(&card.winning_numbers).count();
            (1 << numbers) >> 1
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let cards = parse(input);

    let mut v = vec![0; cards.len()];
    let mut s = 1;
    let mut total = 0;

    for (i, card) in cards.iter().enumerate() {
        total += s;

        let n = card.numbers.intersection(&card.winning_numbers).count();

        v[i + n] += s;
        s = 2*s - v[i];
    }

    total as u32
}

fn main() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    println!("{}", part1(&input));
    println!("{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = include_str!("sample1.in");
        let output = include_str!("sample1.out");

        assert_eq!(format!("{}\n", part1(input)), output);
    }

    #[test]
    fn part2_works() {
        let input = include_str!("sample2.in");
        let output = include_str!("sample2.out");

        assert_eq!(format!("{}\n", part2(input)), output);
    }
}
