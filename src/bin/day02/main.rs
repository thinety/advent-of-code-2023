use std::io::Read;

use anyhow::Result;

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

fn parse(input: &str) -> Vec<Game> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::digit1,
        combinator::{map, map_res, value},
        multi::separated_list1,
        sequence::{delimited, pair, separated_pair},
        IResult,
    };

    fn number(input: &str) -> IResult<&str, u32> {
        map_res(digit1, str::parse)(input)
    }
    fn color(input: &str) -> IResult<&str, Color> {
        alt((
            value(Color::Red, tag("red")),
            value(Color::Green, tag("green")),
            value(Color::Blue, tag("blue")),
        ))(input)
    }
    fn count(input: &str) -> IResult<&str, (u32, Color)> {
        separated_pair(number, tag(" "), color)(input)
    }
    fn counts(input: &str) -> IResult<&str, Vec<(u32, Color)>> {
        separated_list1(tag(", "), count)(input)
    }
    fn round(input: &str) -> IResult<&str, Round> {
        map(counts, |counts| {
            let mut round = Round::default();
            for (n, color) in counts {
                match color {
                    Color::Red => {
                        round.red = n;
                    }
                    Color::Green => {
                        round.green = n;
                    }
                    Color::Blue => {
                        round.blue = n;
                    }
                }
            }
            round
        })(input)
    }
    fn rounds(input: &str) -> IResult<&str, Vec<Round>> {
        separated_list1(tag("; "), round)(input)
    }
    fn game(input: &str) -> IResult<&str, Game> {
        map(
            pair(delimited(tag("Game "), number, tag(": ")), rounds),
            |(id, rounds)| Game { id, rounds },
        )(input)
    }
    fn games(input: &str) -> IResult<&str, Vec<Game>> {
        separated_list1(tag("\n"), game)(input)
    }

    let (input, games) = games(input.trim_end()).unwrap();
    assert!(input.is_empty());
    games
}

fn part1(input: &str) -> u32 {
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

fn part2(input: &str) -> u32 {
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
