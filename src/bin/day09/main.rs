use std::io::Read;

use anyhow::Result;

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| line.split(' ').map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn solve(seq: &[i64]) -> i64 {
    if seq.is_empty() {
        return 0;
    }
    let diff: Vec<_> = (0..seq.len() - 1).map(|i| seq[i + 1] - seq[i]).collect();
    seq.last().unwrap() + solve(&diff)
}

fn part1(input: &str) -> i64 {
    let input = parse(input);
    input.iter().map(|seq| solve(seq)).sum()
}

fn part2(input: &str) -> i64 {
    let input = parse(input);
    input
        .iter()
        .map(|seq| solve(&seq.iter().rev().copied().collect::<Vec<_>>()))
        .sum()
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
