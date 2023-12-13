mod parse;
use parse::parse;

pub fn part1(input: &str) -> u32 {
    let mut input = parse(input);

    fn solve(str: &mut [u8], arr: &[u32], i: usize) -> u32 {
        if i >= str.len() {
            let mut counts = Vec::new();
            let mut count = 0;
            for &c in &*str {
                match c {
                    b'#' => {
                        count += 1;
                    }
                    b'.' => {
                        if count > 0 {
                            counts.push(count);
                            count = 0;
                        }
                    }
                    _ => unreachable!(),
                }
            }
            if count > 0 {
                counts.push(count);
            }
            if counts.len() != arr.len() {
                return 0;
            }
            if !counts.iter().zip(arr.iter()).all(|(n1, n2)| n1 == n2) {
                return 0;
            }
            return 1;
        }

        let mut ans = 0;
        match str[i] {
            b'?' => {
                str[i] = b'.';
                ans += solve(str, arr, i + 1);
                str[i] = b'#';
                ans += solve(str, arr, i + 1);
                str[i] = b'?';
            }
            _ => ans += solve(str, arr, i + 1),
        }
        ans
    }

    input
        .iter_mut()
        .map(|(str, arr)| solve(str, arr, 0))
        .sum()
}

crate::samples! {
    (part1_sample, part1, "sample.in", "21"),
    (part1_puzzle, part1, "puzzle.in", "7916"),
}
