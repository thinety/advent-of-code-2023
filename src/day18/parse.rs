use super::*;

use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_till};
use nom::character::complete::{digit1, multispace0};
use nom::combinator::{all_consuming, map, map_res, value};
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated};
use nom::IResult;

pub(super) fn parse_part1(input: &str) -> Vec<(Direction, u32)> {
    fn line(input: &str) -> IResult<&str, (Direction, u32)> {
        terminated(
            separated_pair(
                alt((
                    value(Direction::North, tag("U")),
                    value(Direction::South, tag("D")),
                    value(Direction::East, tag("R")),
                    value(Direction::West, tag("L")),
                )),
                tag(" "),
                map_res(digit1, str::parse),
            ),
            take_till(|c| c == '\n'),
        )(input)
    }
    fn parse(input: &str) -> IResult<&str, Vec<(Direction, u32)>> {
        separated_list1(tag("\n"), line)(input)
    }

    let (_, output) = all_consuming(terminated(parse, multispace0))(input).unwrap();
    output
}

pub(super) fn parse_part2(input: &str) -> Vec<(Direction, u32)> {
    fn line(input: &str) -> IResult<&str, (Direction, u32)> {
        map(
            preceded(
                take_till(|c| c == '('),
                delimited(
                    tag("(#"),
                    pair(
                        map_res(take(5usize), |digits| u32::from_str_radix(digits, 16)),
                        alt((
                            value(Direction::North, tag("3")),
                            value(Direction::South, tag("1")),
                            value(Direction::East, tag("0")),
                            value(Direction::West, tag("2")),
                        )),
                    ),
                    tag(")"),
                ),
            ),
            |(distance, direction)| (direction, distance),
        )(input)
    }
    fn parse(input: &str) -> IResult<&str, Vec<(Direction, u32)>> {
        separated_list1(tag("\n"), line)(input)
    }

    let (_, output) = all_consuming(terminated(parse, multispace0))(input).unwrap();
    output
}
