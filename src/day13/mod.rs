fn parse(input: &str) -> Vec<Vec<Vec<u8>>> {
    input
        .trim_end()
        .split("\n\n")
        .map(|block| {
            block
                .split('\n')
                .map(|line| line.as_bytes().to_vec())
                .collect()
        })
        .collect()
}

fn horizontal_mirror(input: &Vec<Vec<u8>>, i: usize) -> bool {
    let n = input.len();
    let m = input[0].len();

    let mut ok = true;
    let mut i1 = i;
    let mut i2 = i + 1;
    loop {
        for j in 0..m {
            if input[i1][j] != input[i2][j] {
                ok = false;
            }
        }
        if i1 == 0 || i2 == n - 1 {
            break;
        }
        i1 -= 1;
        i2 += 1;
    }
    ok
}

fn vertical_mirror(input: &Vec<Vec<u8>>, j: usize) -> bool {
    let n = input.len();
    let m = input[0].len();

    let mut ok = true;
    let mut j1 = j;
    let mut j2 = j + 1;
    loop {
        for i in 0..n {
            if input[i][j1] != input[i][j2] {
                ok = false;
            }
        }
        if j1 == 0 || j2 == m - 1 {
            break;
        }
        j1 -= 1;
        j2 += 1;
    }
    ok
}

pub fn part1(input: &str) -> u32 {
    let input = parse(input);

    input
        .iter()
        .map(|block| {
            let n = block.len();
            let m = block[0].len();

            let mut ans = 0;

            for i in 0..n - 1 {
                if horizontal_mirror(block, i) {
                    ans += 100 * ((i as u32) + 1);
                }
            }

            for j in 0..m - 1 {
                if vertical_mirror(block, j) {
                    ans += (j as u32) + 1;
                }
            }

            ans
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let mut input = parse(input);

    input
        .iter_mut()
        .map(|block| {
            let n = block.len();
            let m = block[0].len();

            let mut ans = 0;

            for x in 0..n {
                for y in 0..m {
                    if block[x][y] == b'#' {
                        block[x][y] = b'.';
                    } else {
                        block[x][y] = b'#';
                    }

                    let (i_min, i_max) = if x < n / 2 {
                        (0.max(x), n / 2)
                    } else {
                        (n / 2, (n - 1).min(x))
                    };
                    for i in i_min..i_max {
                        if horizontal_mirror(block, i) {
                            ans += 100 * ((i as u32) + 1);
                        }
                    }

                    let (j_min, j_max) = if y < m / 2 {
                        (0.max(y), m / 2)
                    } else {
                        (m / 2, (m - 1).min(y))
                    };
                    for j in j_min..j_max {
                        if vertical_mirror(block, j) {
                            ans += (j as u32) + 1;
                        }
                    }

                    if block[x][y] == b'#' {
                        block[x][y] = b'.';
                    } else {
                        block[x][y] = b'#';
                    }
                }
            }

            ans
        })
        .sum()
}

crate::samples! {
    (part1_sample, part1, "sample.in", "405"),
    (part1_puzzle, part1, "puzzle.in", "37975"),
    (part2_sample, part2, "sample.in", "400"),
    (part2_puzzle, part2, "puzzle.in", "32497"),
}
