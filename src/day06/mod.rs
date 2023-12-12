mod parse;
use parse::{parse_part1, parse_part2};

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

pub fn part1(input: &str) -> u64 {
    let input = parse_part1(input);

    input
        .iter()
        .map(|&(time, distance)| f(time, distance))
        .product()
}

pub fn part2(input: &str) -> u64 {
    let (time, distance) = parse_part2(input);

    f(time, distance)
}

crate::samples! {
    (part1_sample, part1, "sample.in", "288"),
    (part1_puzzle, part1, "puzzle.in", "303600"),
    (part2_sample, part2, "sample.in", "71503"),
    (part2_puzzle, part2, "puzzle.in", "23654842"),
}
