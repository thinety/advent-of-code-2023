use std::io::Read;

use anyhow::Result;

fn main() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    println!("{}", part1(&input));
    println!("{}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let digits: Vec<_> = s.chars().filter_map(|c| c.to_digit(10)).collect();
            10 * digits.first().unwrap() + digits.last().unwrap()
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let digits: Vec<_> = (0..s.len())
                .filter_map(|i| {
                    let s = &s[i..];
                    Some(if s.starts_with('1') || s.starts_with("one") {
                        1
                    } else if s.starts_with('2') || s.starts_with("two") {
                        2
                    } else if s.starts_with('3') || s.starts_with("three") {
                        3
                    } else if s.starts_with('4') || s.starts_with("four") {
                        4
                    } else if s.starts_with('5') || s.starts_with("five") {
                        5
                    } else if s.starts_with('6') || s.starts_with("six") {
                        6
                    } else if s.starts_with('7') || s.starts_with("seven") {
                        7
                    } else if s.starts_with('8') || s.starts_with("eight") {
                        8
                    } else if s.starts_with('9') || s.starts_with("nine") {
                        9
                    } else {
                        return None;
                    })
                })
                .collect();
            10 * digits.first().unwrap() + digits.last().unwrap()
        })
        .sum()
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
