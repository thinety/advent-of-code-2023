use super::*;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace0, space1};
use nom::combinator::{all_consuming, map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated};
use nom::IResult;

pub(super) fn parse(input: &str) -> Vec<Card> {
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

    let (_, cards) = all_consuming(terminated(cards, multispace0))(input).unwrap();
    cards
}
