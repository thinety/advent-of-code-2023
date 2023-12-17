mod parse;
use parse::parse;

use std::collections::HashMap;

struct Hand {
    cards: [Card; 5],
    r#type: Type,
    bid: u32,
}

impl Hand {
    fn new(cards: [Card; 5], bid: u32) -> Self {
        let mut counts: Vec<u32> = cards
            .iter()
            .fold(HashMap::new(), |mut map, card| {
                *map.entry(card).or_insert(0) += 1;
                map
            })
            .into_values()
            .collect();
        counts.sort();

        let r#type = match counts[..] {
            [5] => Type::FiveOfAKind,
            [1, 4] => Type::FourOfAKind,
            [2, 3] => Type::FullHouse,
            [1, 1, 3] => Type::ThreeOfAKind,
            [1, 2, 2] => Type::TwoPair,
            [1, 1, 1, 2] => Type::OnePair,
            [1, 1, 1, 1, 1] => Type::HighCard,
            _ => unreachable!(),
        };

        Self { cards, r#type, bid }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.r#type.cmp(&other.r#type) {
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
        self.r#type.eq(&other.r#type) && self.cards.eq(&other.cards)
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

pub fn part1(input: &str) -> u32 {
    let mut hands = parse(input);
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| ((i as u32) + 1) * hand.bid)
        .sum()
}
