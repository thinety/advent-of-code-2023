pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let digits: Vec<_> = s.chars().filter_map(|c| c.to_digit(10)).collect();
            10 * digits.first().unwrap() + digits.last().unwrap()
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
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

crate::samples! {
    (part1_sample1, part1, "sample1.in", 142),
    (part1_puzzle, part1, "puzzle.in", 55477),
    (part2_sample2, part2, "sample2.in", 281),
    (part2_puzzle, part2, "puzzle.in", 54431),
}
