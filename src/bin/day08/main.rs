use std::collections::HashMap;
use std::io::Read;

use anyhow::Result;

struct Node {
    left: String,
    right: String,
}

#[derive(Clone)]
enum Direction {
    Left,
    Right,
}

fn parse(input: &str) -> (Vec<Direction>, HashMap<String, Node>) {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::alphanumeric1,
        combinator::{map, value},
        multi::{many1, separated_list1},
        sequence::{delimited, separated_pair},
        IResult,
    };

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

    let (input, (directions, nodes)) = parse(input.trim_end()).unwrap();
    assert!(input.is_empty());
    (directions, nodes)
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut old_r = a;
    let mut r = b;

    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
    }

    old_r
}

fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

fn part1(input: &str) -> u64 {
    let (directions, nodes) = parse(input);

    let mut current = "AAA";
    let mut directions = directions.iter().cycle();
    let mut total = 0;

    while current != "ZZZ" {
        match directions.next().unwrap() {
            Direction::Left => {
                current = &nodes[current].left;
            }
            Direction::Right => {
                current = &nodes[current].right;
            }
        }
        total += 1;
    }

    total
}

fn part2(input: &str) -> u64 {
    let (directions, nodes) = parse(input);

    nodes
        .keys()
        .filter_map(|k| k.ends_with('A').then_some(k.as_ref()))
        .map(|mut current: &str| {
            let mut directions = directions.iter().cycle();
            let mut total = 0;

            while !current.ends_with('Z') {
                match directions.next().unwrap() {
                    Direction::Left => {
                        current = &nodes[current].left;
                    }
                    Direction::Right => {
                        current = &nodes[current].right;
                    }
                }
                total += 1;
            }

            total
        })
        .fold(1, lcm)
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
        let input = include_str!("sample1-1.in");
        let output = include_str!("sample1-1.out");

        assert_eq!(format!("{}\n", part1(input)), output);

        let input = include_str!("sample1-2.in");
        let output = include_str!("sample1-2.out");

        assert_eq!(format!("{}\n", part1(input)), output);
    }

    #[test]
    fn part2_works() {
        let input = include_str!("sample2.in");
        let output = include_str!("sample2.out");

        assert_eq!(format!("{}\n", part2(input)), output);
    }
}
