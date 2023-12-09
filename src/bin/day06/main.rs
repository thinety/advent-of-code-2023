use std::io::Read;

use anyhow::Result;

fn parse1(input: &str) -> Vec<(u64, u64)> {
    use nom::{
        bytes::complete::tag,
        character::complete::{digit1, space1},
        combinator::{map, map_res},
        multi::separated_list1,
        sequence::{pair, preceded, separated_pair},
        IResult,
    };

    fn number(input: &str) -> IResult<&str, u64> {
        map_res(digit1, str::parse)(input)
    }
    fn numbers(input: &str) -> IResult<&str, Vec<u64>> {
        separated_list1(space1, number)(input)
    }
    fn parse(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
        map(
            separated_pair(
                preceded(pair(tag("Time:"), space1), numbers),
                tag("\n"),
                preceded(pair(tag("Distance:"), space1), numbers),
            ),
            |(times, distances)| {
                times
                    .iter()
                    .copied()
                    .zip(distances.iter().copied())
                    .collect()
            },
        )(input)
    }

    let (input, output) = parse(input.trim_end()).unwrap();
    assert!(input.is_empty());
    output
}

fn parse2(input: &str) -> (u64, u64) {
    use nom::{
        bytes::complete::tag,
        character::complete::{digit1, space1},
        combinator::map_res,
        multi::separated_list1,
        sequence::{pair, preceded, separated_pair},
        IResult,
    };

    fn number(input: &str) -> IResult<&str, u64> {
        map_res(separated_list1(space1, digit1), |digits| {
            digits.join("").parse()
        })(input)
    }
    fn parse(input: &str) -> IResult<&str, (u64, u64)> {
        separated_pair(
            preceded(pair(tag("Time:"), space1), number),
            tag("\n"),
            preceded(pair(tag("Distance:"), space1), number),
        )(input)
    }

    let (input, output) = parse(input.trim_end()).unwrap();
    assert!(input.is_empty());
    output
}

fn f(time: u64, distance: u64) -> u64 {
    // d(t) = t*(T-t)
    // d(t) > D
    // d(t) - D > 0
    // -t^2 + Tt - D > 0
    // a = -1, b = T, C = -D

    let time = time as f64;
    let distance = distance as f64;

    let delta_squared = time * time - 4.0 * distance;
    if delta_squared <= 0.0 {
        return 0;
    }

    let delta = delta_squared.sqrt();

    let t1 = (time - delta) / 2.0;
    let t2 = (time + delta) / 2.0;

    let t1 = if t1 == t1.ceil() {
        t1.ceil() + 1.0
    } else {
        t1.ceil()
    };
    let t2 = t2.ceil();

    (t2 - t1) as u64
}

fn part1(input: &str) -> u64 {
    let input = parse1(input);

    input
        .iter()
        .map(|&(time, distance)| f(time, distance))
        .product()
}

fn part2(input: &str) -> u64 {
    let (time, distance) = parse2(input);

    f(time, distance)
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
