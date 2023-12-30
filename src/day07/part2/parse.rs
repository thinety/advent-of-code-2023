use super::*;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace0};
use nom::combinator::{all_consuming, map, map_res, value};
use nom::multi::separated_list1;
use nom::sequence::{terminated, tuple};
use nom::IResult;

pub(super) fn parse(input: &str) -> Vec<Hand> {
    fn number(input: &str) -> IResult<&str, u32> {
        map_res(digit1, str::parse)(input)
    }
    fn card(input: &str) -> IResult<&str, Card> {
        alt((
            value(Card::Joker, tag("J")),
            value(Card::Two, tag("2")),
            value(Card::Three, tag("3")),
            value(Card::Four, tag("4")),
            value(Card::Five, tag("5")),
            value(Card::Six, tag("6")),
            value(Card::Seven, tag("7")),
            value(Card::Eight, tag("8")),
            value(Card::Nine, tag("9")),
            value(Card::Ten, tag("T")),
            value(Card::Queen, tag("Q")),
            value(Card::King, tag("K")),
            value(Card::Ace, tag("A")),
        ))(input)
    }
    fn hand(input: &str) -> IResult<&str, Hand> {
        map(
            tuple((card, card, card, card, card, tag(" "), number)),
            |(c1, c2, c3, c4, c5, _, bid)| Hand::new([c1, c2, c3, c4, c5], bid),
        )(input)
    }
    fn hands(input: &str) -> IResult<&str, Vec<Hand>> {
        separated_list1(tag("\n"), hand)(input)
    }

    let (_, hands) = all_consuming(terminated(hands, multispace0))(input).unwrap();
    hands
}
