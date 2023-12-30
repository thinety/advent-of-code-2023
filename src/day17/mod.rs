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

struct State {
    i: usize,
    j: usize,
    d: Direction,
    k: usize,
    c: u32,
}

impl Eq for State {}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.c == other.c
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.c.cmp(&self.c)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(input: &str, min_k: usize, max_k: usize) -> u32 {
    let input = parse(input);

    let n = input.len();
    let m = input[0].len();

    let mut priority_queue = BinaryHeap::new();
    let mut visited = vec![vec![vec![vec![false; max_k]; 4]; m]; n];

    priority_queue.push(State {
        i: 0,
        j: 1,
        d: Direction::East,
        k: 0,
        c: input[0][1],
    });
    priority_queue.push(State {
        i: 1,
        j: 0,
        d: Direction::South,
        k: 0,
        c: input[1][0],
    });

    while let Some(State { i, j, d, k, c }) = priority_queue.pop() {
        if visited[i][j][d as usize][k] {
            continue;
        }
        visited[i][j][d as usize][k] = true;

        if i == n - 1 && j == m - 1 && k >= min_k - 1 {
            return c;
        }

        match d {
            Direction::North => {
                if k < max_k - 1 && i > 0 {
                    priority_queue.push(State {
                        i: i - 1,
                        j: j,
                        d: Direction::North,
                        k: k + 1,
                        c: c + input[i - 1][j],
                    });
                }
                if k >= min_k - 1 && j > 0 {
                    priority_queue.push(State {
                        i: i,
                        j: j - 1,
                        d: Direction::West,
                        k: 0,
                        c: c + input[i][j - 1],
                    });
                }
                if k >= min_k - 1 && j < m - 1 {
                    priority_queue.push(State {
                        i: i,
                        j: j + 1,
                        d: Direction::East,
                        k: 0,
                        c: c + input[i][j + 1],
                    });
                }
            }
            Direction::South => {
                if k < max_k - 1 && i < n - 1 {
                    priority_queue.push(State {
                        i: i + 1,
                        j: j,
                        d: Direction::South,
                        k: k + 1,
                        c: c + input[i + 1][j],
                    });
                }
                if k >= min_k - 1 && j > 0 {
                    priority_queue.push(State {
                        i: i,
                        j: j - 1,
                        d: Direction::West,
                        k: 0,
                        c: c + input[i][j - 1],
                    });
                }
                if k >= min_k - 1 && j < m - 1 {
                    priority_queue.push(State {
                        i: i,
                        j: j + 1,
                        d: Direction::East,
                        k: 0,
                        c: c + input[i][j + 1],
                    });
                }
            }
            Direction::East => {
                if k < max_k - 1 && j < m - 1 {
                    priority_queue.push(State {
                        i: i,
                        j: j + 1,
                        d: Direction::East,
                        k: k + 1,
                        c: c + input[i][j + 1],
                    });
                }
                if k >= min_k - 1 && i > 0 {
                    priority_queue.push(State {
                        i: i - 1,
                        j: j,
                        d: Direction::North,
                        k: 0,
                        c: c + input[i - 1][j],
                    });
                }
                if k >= min_k - 1 && i < n - 1 {
                    priority_queue.push(State {
                        i: i + 1,
                        j: j,
                        d: Direction::South,
                        k: 0,
                        c: c + input[i + 1][j],
                    });
                }
            }
            Direction::West => {
                if k < max_k - 1 && j > 0 {
                    priority_queue.push(State {
                        i: i,
                        j: j - 1,
                        d: Direction::West,
                        k: k + 1,
                        c: c + input[i][j - 1],
                    });
                }
                if k >= min_k - 1 && i > 0 {
                    priority_queue.push(State {
                        i: i - 1,
                        j: j,
                        d: Direction::North,
                        k: 0,
                        c: c + input[i - 1][j],
                    });
                }
                if k >= min_k - 1 && i < n - 1 {
                    priority_queue.push(State {
                        i: i + 1,
                        j: j,
                        d: Direction::South,
                        k: 0,
                        c: c + input[i + 1][j],
                    });
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
