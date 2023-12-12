mod parse;
use parse::parse;

fn solve(seq: &[i64]) -> i64 {
    if seq.is_empty() {
        return 0;
    }
    let diff: Vec<_> = (0..seq.len() - 1).map(|i| seq[i + 1] - seq[i]).collect();
    seq.last().unwrap() + solve(&diff)
}

pub fn part1(input: &str) -> i64 {
    let input = parse(input);
    input.iter().map(|seq| solve(seq)).sum()
}

pub fn part2(input: &str) -> i64 {
    let mut input = parse(input);
    input.iter_mut().for_each(|seq| seq.reverse());
    input.iter().map(|seq| solve(seq)).sum()
}

crate::samples! {
    (part1_sample, part1, "sample.in", "114"),
    (part1_puzzle, part1, "puzzle.in", "2174807968"),
    (part2_sample, part2, "sample.in", "2"),
    (part2_puzzle, part2, "puzzle.in", "1208"),
}
