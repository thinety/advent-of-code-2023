use std::collections::VecDeque;

#[derive(Clone, Copy)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
}

#[derive(Clone)]
enum Tile {
    Ground,
    Pipe(Pipe),
}

fn solve(input: &str) -> (u32, u32) {
    let mut input = input
        .lines()
        .map(|s| s.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let n = input.len();
    let m = input[0].len();

    let (i, j) = {
        let mut start = None;
        for i in 0..n {
            for j in 0..m {
                if let b'S' = input[i][j] {
                    start = Some((i, j));
                }
            }
        }
        let (i, j) = start.unwrap();

        let (mut north, mut south, mut east, mut west) = (false, false, false, false);
        if i > 0 {
            if let b'|' | b'7' | b'F' = input[i - 1][j] {
                north = true;
            }
        }
        if i + 1 < n {
            if let b'|' | b'L' | b'J' = input[i + 1][j] {
                south = true;
            }
        }
        if j + 1 < m {
            if let b'-' | b'J' | b'7' = input[i][j + 1] {
                east = true;
            }
        }
        if j > 0 {
            if let b'-' | b'L' | b'F' = input[i][j - 1] {
                west = true;
            }
        }
        match (north, south, east, west) {
            (true, true, false, false) => {
                input[i][j] = b'|';
            }
            (false, false, true, true) => {
                input[i][j] = b'-';
            }
            (true, false, true, false) => {
                input[i][j] = b'L';
            }
            (true, false, false, true) => {
                input[i][j] = b'J';
            }
            (false, true, false, true) => {
                input[i][j] = b'7';
            }
            (false, true, true, false) => {
                input[i][j] = b'F';
            }
            _ => unreachable!(),
        }

        (i, j)
    };

    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; m]; n];
    let mut grid = vec![vec![Tile::Ground; m]; n];
    let mut d_max = 0;

    queue.push_back((i, j, 0));
    while let Some((i, j, d)) = queue.pop_front() {
        if visited[i][j] {
            continue;
        }
        visited[i][j] = true;

        let pipe = match input[i][j] {
            b'|' => Pipe::NS,
            b'-' => Pipe::EW,
            b'L' => Pipe::NE,
            b'J' => Pipe::NW,
            b'7' => Pipe::SW,
            b'F' => Pipe::SE,
            _ => unreachable!(),
        };

        grid[i][j] = Tile::Pipe(pipe);
        d_max = d_max.max(d);

        match pipe {
            Pipe::NS => {
                queue.push_back((i - 1, j, d + 1));
                queue.push_back((i + 1, j, d + 1));
            }
            Pipe::EW => {
                queue.push_back((i, j - 1, d + 1));
                queue.push_back((i, j + 1, d + 1));
            }
            Pipe::NE => {
                queue.push_back((i - 1, j, d + 1));
                queue.push_back((i, j + 1, d + 1));
            }
            Pipe::NW => {
                queue.push_back((i - 1, j, d + 1));
                queue.push_back((i, j - 1, d + 1));
            }
            Pipe::SW => {
                queue.push_back((i + 1, j, d + 1));
                queue.push_back((i, j - 1, d + 1));
            }
            Pipe::SE => {
                queue.push_back((i + 1, j, d + 1));
                queue.push_back((i, j + 1, d + 1));
            }
        }
    }

    let mut inside_count = 0;

    // https://en.wikipedia.org/wiki/Even-odd_rule
    for i in 0..n {
        let mut inside = false;
        for j in 0..m {
            match grid[i][j] {
                Tile::Ground => {
                    if inside {
                        inside_count += 1;
                    }
                }
                Tile::Pipe(Pipe::NS | Pipe::NE | Pipe::NW) => {
                    inside = !inside;
                }
                Tile::Pipe(Pipe::EW | Pipe::SE | Pipe::SW) => {}
            }
        }
    }

    (d_max, inside_count)
}

pub fn part1(input: &str) -> u32 {
    let (ans, _) = solve(input);
    ans
}

pub fn part2(input: &str) -> u32 {
    let (_, ans) = solve(input);
    ans
}

crate::samples! {
    (part1, part1_sample1, "sample1.in", "sample1.out1"),
    (part1, part1_sample2, "sample2.in", "sample2.out1"),
    (part1, part1_puzzle, "puzzle.in", "puzzle.out1"),
    (part2, part2_sample3, "sample3.in", "sample3.out2"),
    (part2, part2_sample4, "sample4.in", "sample4.out2"),
    (part2, part2_sample5, "sample5.in", "sample5.out2"),
    (part2, part2_sample6, "sample6.in", "sample6.out2"),
    (part2, part2_puzzle, "puzzle.in", "puzzle.out2"),
}
