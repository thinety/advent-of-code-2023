mod parse;
use parse::{parse_part1, parse_part2};

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

pub fn part1(input: &str) -> u64 {
    let (seeds, mappings) = parse_part1(input);

    seeds
        .iter()
        .map(|&seed| mappings.iter().fold(seed, |x, mapping| mapping.map(x)))
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> u64 {
    let (seeds, mappings) = parse_part2(input);

    seeds
        .iter()
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

crate::samples! {
    (part1, part1_sample, "sample.in", "sample.out1"),
    (part1, part1_puzzle, "puzzle.in", "puzzle.out1"),
    (part2, part2_sample, "sample.in", "sample.out2"),
    //(part2, part2_puzzle, "puzzle.in", "puzzle.out2"),
}
