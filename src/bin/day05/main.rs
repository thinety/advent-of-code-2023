use std::io::Read;

use anyhow::Result;

use rayon::prelude::*;

struct Mapping {
    ranges: Vec<Range>,
}

impl Mapping {
    fn map(&self, x: u64) -> u64 {
        for range in &self.ranges {
            let src = range.source;
            let dst = range.destination;
            let len = range.length;

            if (src..src + len).contains(&x) {
                return dst + (x - src);
            }
        }
        x
    }
}

struct Range {
    source: u64,
    destination: u64,
    length: u64,
}

fn parse1(input: &str) -> (Vec<u64>, Vec<Mapping>) {
    use nom::{
        bytes::complete::{tag, take_until1},
        character::complete::digit1,
        combinator::{map, map_res},
        multi::separated_list1,
        sequence::{preceded, separated_pair, terminated, tuple},
        IResult,
    };

    fn number(input: &str) -> IResult<&str, u64> {
        map_res(digit1, str::parse)(input)
    }
    fn range(input: &str) -> IResult<&str, Range> {
        map(
            tuple((number, tag(" "), number, tag(" "), number)),
            |(destination, _, source, _, length)| Range {
                source,
                destination,
                length,
            },
        )(input)
    }
    fn ranges(input: &str) -> IResult<&str, Vec<Range>> {
        separated_list1(tag("\n"), range)(input)
    }
    fn mapping(input: &str) -> IResult<&str, Mapping> {
        map(
            preceded(terminated(take_until1("\n"), tag("\n")), ranges),
            |ranges| Mapping { ranges },
        )(input)
    }
    fn mappings(input: &str) -> IResult<&str, Vec<Mapping>> {
        separated_list1(tag("\n\n"), mapping)(input)
    }
    fn seeds(input: &str) -> IResult<&str, Vec<u64>> {
        preceded(tag("seeds: "), separated_list1(tag(" "), number))(input)
    }
    fn parse(input: &str) -> IResult<&str, (Vec<u64>, Vec<Mapping>)> {
        separated_pair(seeds, tag("\n\n"), mappings)(input)
    }

    let (input, (seeds, mappings)) = parse(input.trim_end()).unwrap();
    assert!(input.is_empty());
    (seeds, mappings)
}

fn parse2(input: &str) -> (Vec<(u64, u64)>, Vec<Mapping>) {
    use nom::{
        bytes::complete::{tag, take_until1},
        character::complete::digit1,
        combinator::{map, map_res},
        multi::separated_list1,
        sequence::{preceded, separated_pair, terminated, tuple},
        IResult,
    };

    fn number(input: &str) -> IResult<&str, u64> {
        map_res(digit1, str::parse)(input)
    }
    fn range(input: &str) -> IResult<&str, Range> {
        map(
            tuple((number, tag(" "), number, tag(" "), number)),
            |(destination, _, source, _, length)| Range {
                source,
                destination,
                length,
            },
        )(input)
    }
    fn ranges(input: &str) -> IResult<&str, Vec<Range>> {
        separated_list1(tag("\n"), range)(input)
    }
    fn mapping(input: &str) -> IResult<&str, Mapping> {
        map(
            preceded(terminated(take_until1("\n"), tag("\n")), ranges),
            |ranges| Mapping { ranges },
        )(input)
    }
    fn mappings(input: &str) -> IResult<&str, Vec<Mapping>> {
        separated_list1(tag("\n\n"), mapping)(input)
    }
    fn seeds(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
        preceded(tag("seeds: "), separated_list1(tag(" "), separated_pair(number, tag(" "), number)))(input)
    }
    fn parse(input: &str) -> IResult<&str, (Vec<(u64, u64)>, Vec<Mapping>)> {
        separated_pair(seeds, tag("\n\n"), mappings)(input)
    }

    let (input, (seeds, mappings)) = parse(input.trim_end()).unwrap();
    assert!(input.is_empty());
    (seeds, mappings)
}

fn part1(input: &str) -> u64 {
    let (seeds, mappings) = parse1(input);

    seeds
        .iter()
        .map(|&seed| mappings.iter().fold(seed, |x, mapping| mapping.map(x)))
        .min()
        .unwrap()
}

fn part2(input: &str) -> u64 {
    let (seeds, mappings) = parse2(input);

    seeds.iter()
        .map(|&(start, length)| {
            (start..start + length)
                .into_par_iter()
                .map(|seed| mappings.iter().fold(seed, |x, mapping| mapping.map(x)))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
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
