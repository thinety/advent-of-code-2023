use std::collections::HashMap;

struct Hand {
    cards: [Card; 5],
    bid: u64,
}

impl Hand {
    fn r#type(&self) -> Type {
        let mut counts: Vec<u32> = self
            .cards
            .iter()
            .fold(HashMap::new(), |mut map, card| {
                *map.entry(card).or_insert(0) += 1;
                map
            })
            .into_values()
            .collect();
        counts.sort();

        match counts[..] {
            [5] => Type::FiveOfAKind,
            [1, 4] => Type::FourOfAKind,
            [2, 3] => Type::FullHouse,
            [1, 1, 3] => Type::ThreeOfAKind,
            [1, 2, 2] => Type::TwoPair,
            [1, 1, 1, 2] => Type::OnePair,
            [1, 1, 1, 1, 1] => Type::HighCard,
            _ => unreachable!(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.r#type().cmp(&other.r#type()) {
            std::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
            ordering => ordering,
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Hand {}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.r#type().eq(&other.r#type()) && self.cards.eq(&other.cards)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn parse(input: &str) -> Vec<Hand> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::digit1,
        combinator::{map, map_res, value},
        multi::separated_list1,
        sequence::tuple,
        IResult,
    };

    fn number(input: &str) -> IResult<&str, u64> {
        map_res(digit1, str::parse)(input)
    }
    fn card(input: &str) -> IResult<&str, Card> {
        alt((
            value(Card::Two, tag("2")),
            value(Card::Three, tag("3")),
            value(Card::Four, tag("4")),
            value(Card::Five, tag("5")),
            value(Card::Six, tag("6")),
            value(Card::Seven, tag("7")),
            value(Card::Eight, tag("8")),
            value(Card::Nine, tag("9")),
            value(Card::Ten, tag("T")),
            value(Card::Jack, tag("J")),
            value(Card::Queen, tag("Q")),
            value(Card::King, tag("K")),
            value(Card::Ace, tag("A")),
        ))(input)
    }
    fn hand(input: &str) -> IResult<&str, Hand> {
        map(
            tuple((card, card, card, card, card, tag(" "), number)),
            |(c1, c2, c3, c4, c5, _, bid)| Hand {
                cards: [c1, c2, c3, c4, c5],
                bid,
            },
        )(input)
    }
    fn hands(input: &str) -> IResult<&str, Vec<Hand>> {
        separated_list1(tag("\n"), hand)(input)
    }

    let (input, hands) = hands(input.trim_end()).unwrap();
    assert!(input.is_empty());
    hands
}

pub fn part1(input: &str) -> u64 {
    let mut hands = parse(input);
    hands.sort();
    hands.iter().enumerate().map(|(i, hand)| {
        (i as u64 +1) * hand.bid
    }).sum()
}
