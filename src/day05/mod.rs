mod parse;
use parse::{parse_part1, parse_part2};

pub fn part1(input: &str) -> u64 {
    let (seeds, mappings) = parse_part1(input);

    seeds
        .iter()
        .map(|&seed| {
            mappings.iter().fold(seed, |x, mapping| {
                for &(start, end, offset) in mapping {
                    if (start..end).contains(&x) {
                        return offset + (x - start);
                    }
                }
                x
            })
        })
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> u64 {
    let (seeds, mut mappings) = parse_part2(input);

    mappings.iter_mut().for_each(|mapping| mapping.sort());

    let mut ranges = seeds;
    for mapping in &mappings {
        let mut new_ranges = Vec::new();

        for &(mut start, end) in &ranges {
            let mut last_map_end = 0;

            for i in 0..mapping.len() {
                let (map_start, map_end, map_offset) = mapping[i];

                if last_map_end <= start && start < map_start {
                    let new_start = end.min(map_start);
                    new_ranges.push((start, new_start));
                    start = new_start;
                }

                if map_start <= start && start < map_end {
                    let new_start = end.min(map_end);
                    new_ranges.push((
                        map_offset + (start - map_start),
                        map_offset + (new_start - map_start),
                    ));
                    start = new_start;
                }

                last_map_end = map_end;
            }

            if last_map_end <= start {
                new_ranges.push((start, end));
            }
        }

        ranges = new_ranges;
    }

    ranges.iter().map(|&(start, _)| start).min().unwrap()
}

crate::samples! {
    (part1_sample, part1, "sample.in", 35),
    (part1_puzzle, part1, "puzzle.in", 486613012),
    (part2_sample, part2, "sample.in", 46),
    (part2_puzzle, part2, "puzzle.in", 56931769),
}
