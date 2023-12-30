use super::*;

use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, multispace0};
use nom::combinator::{map, value, all_consuming};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated};
use nom::IResult;

pub(super) fn parse(input: &str) -> (Vec<Direction>, HashMap<String, Node>) {
    fn nodes(input: &str) -> IResult<&str, HashMap<String, Node>> {
        map(
            separated_list1(
                tag("\n"),
                separated_pair(
                    alphanumeric1,
                    tag(" = "),
                    delimited(
                        tag("("),
                        separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                        tag(")"),
                    ),
                ),
            ),
            |list| {
                list.into_iter()
                    .map(|(key, (left, right)): (&str, _)| {
                        (
                            key.to_string(),
                            Node {
                                left: left.to_string(),
                                right: right.to_string(),
                            },
                        )
                    })
                    .collect()
            },
        )(input)
    }
    fn direction(input: &str) -> IResult<&str, Direction> {
        alt((
            value(Direction::Left, tag("L")),
            value(Direction::Right, tag("R")),
        ))(input)
    }
    fn directions(input: &str) -> IResult<&str, Vec<Direction>> {
        many1(direction)(input)
    }
    fn parse(input: &str) -> IResult<&str, (Vec<Direction>, HashMap<String, Node>)> {
        separated_pair(directions, tag("\n\n"), nodes)(input)
    }

    let (_, (directions, nodes)) = all_consuming(terminated(parse, multispace0))(input).unwrap();
    (directions, nodes)
}
