pub fn solve(input: &str, expansion_factor: u64) -> u64 {
    let input = input
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let n = input.len();
    let m = input[0].len();

    let mut empty_row = vec![true; n];
    let mut empty_col = vec![true; m];

    let mut galaxies = Vec::new();
    for i in 0..n {
        for j in 0..m {
            if input[i][j] == b'#' {
                empty_row[i] = false;
                empty_col[j] = false;
                galaxies.push((i, j));
            }
        }
    }

    let mut prefix_row = vec![0; n + 1];
    for i in 0..n {
        prefix_row[i + 1] = prefix_row[i];
        if empty_row[i] {
            prefix_row[i + 1] += expansion_factor - 1;
        }
    }

    let mut prefix_col = vec![0; m + 1];
    for j in 0..m {
        prefix_col[j + 1] = prefix_col[j];
        if empty_col[j] {
            prefix_col[j + 1] += expansion_factor - 1;
        }
    }

    let mut ans = 0;
    for (i, &(x1, y1)) in galaxies.iter().enumerate() {
        let x1 = (x1 as u64) + prefix_row[x1 + 1];
        let y1 = (y1 as u64) + prefix_col[y1 + 1];

        for &(x2, y2) in galaxies[i + 1..].iter() {
            let x2 = (x2 as u64) + prefix_row[x2 + 1];
            let y2 = (y2 as u64) + prefix_col[y2 + 1];

            ans += x2.abs_diff(x1) + y2.abs_diff(y1);
        }
    }

    ans
}

pub fn part1(input: &str) -> u64 {
    solve(input, 2)
}

pub fn part2(input: &str) -> u64 {
    solve(input, 1000000)
}

crate::samples! {
    (part1_sample, part1, "sample.in", "374"),
    (part1_puzzle, part1, "puzzle.in", "9734203"),
    (part2_sample1, |input| solve(input,  10), "sample.in", "1030"),
    (part2_sample2, |input| solve(input, 100), "sample.in", "8410"),
    (part2_puzzle, part2, "puzzle.in", "568914596391"),
}
