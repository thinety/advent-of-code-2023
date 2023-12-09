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
    (part1, part1_sample, "sample.in", "sample.out1"),
    (part1, part1_puzzle, "puzzle.in", "puzzle.out1"),
    (part2, part2_sample, "sample.in", "sample.out2"),
    (part2, part2_puzzle, "puzzle.in", "puzzle.out2"),
}
