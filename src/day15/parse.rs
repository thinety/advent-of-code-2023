use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::{map, map_res, value};
use nom::multi::separated_list1;
use nom::sequence::{pair, preceded};
use nom::IResult;

use super::*;

pub(super) fn parse(input: &str) -> Vec<Step> {
    fn number(input: &str) -> IResult<&str, u32> {
        map_res(digit1, str::parse)(input)
    }
    fn operation(input: &str) -> IResult<&str, Operation> {
        alt((
            value(Operation::Dash, tag("-")),
            map(preceded(tag("="), number), Operation::Equal),
        ))(input)
    }
    fn step(input: &str) -> IResult<&str, Step> {
        map(pair(alpha1, operation), |(label, operation)| Step {
            label: label.to_string(),
            operation,
        })(input)
    }
    fn steps(input: &str) -> IResult<&str, Vec<Step>> {
        separated_list1(tag(","), step)(input)
    }

    let (input, cards) = steps(input.trim_end()).unwrap();
    assert!(input.is_empty());
    cards
}
