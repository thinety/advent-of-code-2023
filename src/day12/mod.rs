mod parse;
use parse::parse;

fn solve(str: &[u8], arr: &[usize]) -> u64 {
    let mut dots = vec![0; str.len() + 1];
    for i in 0..str.len() {
        dots[i + 1] = dots[i];
        if str[i] == b'.' {
            dots[i + 1] += 1;
        }
    }

    let mut dp = vec![vec![0; arr.len() + 1]; str.len()];
    for i in 0..str.len() {
        for j in 0..arr.len() + 1 {
            if str[i] == b'.' || str[i] == b'?' {
                if i > 0 {
                    dp[i][j] += dp[i - 1][j];
                } else if j == 0 {
                    dp[i][j] += 1;
                }
            }

            if (str[i] == b'#' || str[i] == b'?')
                && (j > 0)
                && (i + 1 >= arr[j - 1])
                && ((dots[i + 1] - dots[i + 1 - arr[j - 1]]) == 0)
            {
                if i >= arr[j - 1] {
                    if str[i - arr[j - 1]] != b'#' {
                        if i - 1 >= arr[j - 1] {
                            dp[i][j] += dp[i - 1 - arr[j - 1]][j - 1];
                        } else if j == 1 {
                            dp[i][j] += 1;
                        }
                    }
                } else if j == 1 {
                    dp[i][j] += 1;
                }
            }
        }
    }

    dp[str.len() - 1][arr.len()]
}

pub fn part1(input: &str) -> u64 {
    let input = parse(input);

    input.iter().map(|(str, arr)| solve(str, arr)).sum()
}

pub fn part2(input: &str) -> u64 {
    let input = parse(input);

    input
        .iter()
        .map(|(str, arr)| {
            (
                str.iter()
                    .copied()
                    .chain(std::iter::once(b'?'))
                    .cycle()
                    .take(5 * (str.len() + 1) - 1)
                    .collect::<Vec<_>>(),
                arr.iter()
                    .copied()
                    .cycle()
                    .take(5 * arr.len())
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(str, arr)| solve(&str, &arr))
        .sum()
}

crate::samples! {
    (part1_sample, part1, "sample.in", "21"),
    (part1_puzzle, part1, "puzzle.in", "7916"),
    (part2_sample, part2, "sample.in", "525152"),
    (part2_puzzle, part2, "puzzle.in", "37366887898686"),
}
