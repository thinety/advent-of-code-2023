use std::collections::BinaryHeap;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|d| d.to_digit(10).unwrap()).collect())
        .collect()
}

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
enum Direction {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

fn solve(input: &str, min_k: usize, max_k: usize) -> u32 {
    let input = parse(input);

    let n = input.len();
    let m = input[0].len();

    let mut priority_queue = BinaryHeap::new();
    let mut visited = vec![vec![vec![vec![false; max_k]; 4]; m]; n];

    priority_queue.push((-(input[0][1] as i32), 0, 1, Direction::East, 0));
    priority_queue.push((-(input[1][0] as i32), 1, 0, Direction::South, 0));

    while let Some((c, i, j, d, k)) = priority_queue.pop() {
        if visited[i][j][d as usize][k] {
            continue;
        }
        visited[i][j][d as usize][k] = true;

        if i == n - 1 && j == m - 1 && k >= min_k - 1 {
            return -c as u32;
        }

        match d {
            Direction::North => {
                if k < max_k - 1 && i > 0 {
                    priority_queue.push((
                        c - (input[i - 1][j] as i32),
                        i - 1,
                        j,
                        Direction::North,
                        k + 1,
                    ));
                }
                if k >= min_k - 1 && j > 0 {
                    priority_queue.push((
                        c - (input[i][j - 1] as i32),
                        i,
                        j - 1,
                        Direction::West,
                        0,
                    ));
                }
                if k >= min_k - 1 && j < m - 1 {
                    priority_queue.push((
                        c - (input[i][j + 1] as i32),
                        i,
                        j + 1,
                        Direction::East,
                        0,
                    ));
                }
            }
            Direction::South => {
                if k < max_k - 1 && i < n - 1 {
                    priority_queue.push((
                        c - (input[i + 1][j] as i32),
                        i + 1,
                        j,
                        Direction::South,
                        k + 1,
                    ));
                }
                if k >= min_k - 1 && j > 0 {
                    priority_queue.push((
                        c - (input[i][j - 1] as i32),
                        i,
                        j - 1,
                        Direction::West,
                        0,
                    ));
                }
                if k >= min_k - 1 && j < m - 1 {
                    priority_queue.push((
                        c - (input[i][j + 1] as i32),
                        i,
                        j + 1,
                        Direction::East,
                        0,
                    ));
                }
            }
            Direction::East => {
                if k < max_k - 1 && j < m - 1 {
                    priority_queue.push((
                        c - (input[i][j + 1] as i32),
                        i,
                        j + 1,
                        Direction::East,
                        k + 1,
                    ));
                }
                if k >= min_k - 1 && i > 0 {
                    priority_queue.push((
                        c - (input[i - 1][j] as i32),
                        i - 1,
                        j,
                        Direction::North,
                        0,
                    ));
                }
                if k >= min_k - 1 && i < n - 1 {
                    priority_queue.push((
                        c - (input[i + 1][j] as i32),
                        i + 1,
                        j,
                        Direction::South,
                        0,
                    ));
                }
            }
            Direction::West => {
                if k < max_k - 1 && j > 0 {
                    priority_queue.push((
                        c - (input[i][j - 1] as i32),
                        i,
                        j - 1,
                        Direction::West,
                        k + 1,
                    ));
                }
                if k >= min_k - 1 && i > 0 {
                    priority_queue.push((
                        c - (input[i - 1][j] as i32),
                        i - 1,
                        j,
                        Direction::North,
                        0,
                    ));
                }
                if k >= min_k - 1 && i < n - 1 {
                    priority_queue.push((
                        c - (input[i + 1][j] as i32),
                        i + 1,
                        j,
                        Direction::South,
                        0,
                    ));
                }
            }
        }
    }

    panic!()
}

pub fn part1(input: &str) -> u32 {
    solve(input, 1, 3)
}

pub fn part2(input: &str) -> u32 {
    solve(input, 4, 10)
}

crate::samples! {
    (part1_sample1, part1, "sample1.in", 102),
    (part1_puzzle, part1, "puzzle.in", 724),
    (part2_sample1, part2, "sample1.in", 94),
    (part2_sample2, part2, "sample2.in", 71),
    (part2_puzzle, part2, "puzzle.in", 877),
}
