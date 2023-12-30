use super::*;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace0};
use nom::combinator::{map, map_res, value, all_consuming};
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, separated_pair, terminated};
use nom::IResult;

pub(super) fn parse(input: &str) -> Vec<Game> {
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

    let (_, games) = all_consuming(terminated(games, multispace0))(input).unwrap();
    games
}
