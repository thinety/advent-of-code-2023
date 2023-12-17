use std::hash::{Hash, Hasher};

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn hash(input: &Vec<Vec<u8>>) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

fn north(input: &mut Vec<Vec<u8>>) {
    let n = input.len();
    let m = input[0].len();

    for j in 0..m {
        let mut last = 0;
        for i in 0..n {
            if input[i][j] == b'#' {
                last = i + 1;
            }
            if input[i][j] == b'O' {
                input[i][j] = b'.';
                input[last][j] = b'O';
                last += 1;
            }
        }
    }
}
fn west(input: &mut Vec<Vec<u8>>) {
    let n = input.len();
    let m = input[0].len();

    for i in 0..n {
        let mut last = 0;
        for j in 0..m {
            if input[i][j] == b'#' {
                last = j + 1;
            }
            if input[i][j] == b'O' {
                input[i][j] = b'.';
                input[i][last] = b'O';
                last += 1;
            }
        }
    }
}
fn south(input: &mut Vec<Vec<u8>>) {
    let n = input.len();
    let m = input[0].len();

    for j in 0..m {
        let mut last = 0;
        for i in 0..n {
            if input[n - 1 - i][j] == b'#' {
                last = i + 1;
            }
            if input[n - 1 - i][j] == b'O' {
                input[n - 1 - i][j] = b'.';
                input[n - 1 - last][j] = b'O';
                last += 1;
            }
        }
    }
}
fn east(input: &mut Vec<Vec<u8>>) {
    let n = input.len();
    let m = input[0].len();

    for i in 0..n {
        let mut last = 0;
        for j in 0..m {
            if input[i][m - 1 - j] == b'#' {
                last = j + 1;
            }
            if input[i][m - 1 - j] == b'O' {
                input[i][m - 1 - j] = b'.';
                input[i][m - 1 - last] = b'O';
                last += 1;
            }
        }
    }
}

fn solve(input: &Vec<Vec<u8>>) -> u32 {
    let n = input.len();
    let m = input[0].len();

    let mut ans = 0;
    for j in 0..m {
        for i in 0..n {
            if input[i][j] == b'O' {
                ans += (n - i) as u32;
            }
        }
    }
    ans
}

pub fn part1(input: &str) -> u32 {
    let mut input = parse(input);

    north(&mut input);
    solve(&input)
}

pub fn part2(input: &str) -> u32 {
    let mut input = parse(input);

    let (o, p) = {
        let mut states = std::collections::HashMap::new();
        states.insert(hash(&input), 0);

        let mut i = 0;
        loop {
            north(&mut input);
            west(&mut input);
            south(&mut input);
            east(&mut input);
            i += 1;
            if let Some(o) = states.insert(hash(&input), i) {
                break (o, i - o);
            }
        }
    };

    let x = 1000000000;
    for _ in 0..((x - o) % p) {
        north(&mut input);
        west(&mut input);
        south(&mut input);
        east(&mut input);
    }

    solve(&input)
}

crate::samples! {
    (part1_sample, part1, "sample.in", "136"),
    (part1_puzzle, part1, "puzzle.in", "105784"),
    (part2_sample, part2, "sample.in", "64"),
    (part2_puzzle, part2, "puzzle.in", "91286"),
}
