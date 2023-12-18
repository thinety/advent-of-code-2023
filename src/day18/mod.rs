mod parse;
use std::iter::once;

use parse::{parse_part1, parse_part2};

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Point(i64, i64);

impl Point {
    fn cross(&self, other: &Point) -> i64 {
        self.0 * other.1 - other.0 * self.1
    }
}

fn solve(input: &[(Direction, u32)]) -> i64 {
    let mut right_points = Vec::new();
    let mut left_points = Vec::new();

    let (mut x, mut y) = {
        let (dir, _) = input[0];
        match dir {
            Direction::North => (1, 0),
            Direction::South => (0, -1),
            Direction::East => (1, -1),
            Direction::West => (0, 0),
        }
    };

    for i in 0..input.len() {
        let ii = if i < input.len() - 1 { i + 1 } else { 0 };

        let (dir, dis) = input[i];
        let (next_dir, _) = input[ii];

        match dir {
            Direction::North => {
                y += (dis - 1) as i64;
                match next_dir {
                    Direction::East => {
                        right_points.push(Point(x, y));
                        left_points.push(Point(x - 1, y + 1));
                    }
                    Direction::West => {
                        y += 1;
                        right_points.push(Point(x, y));
                        x -= 1;
                        left_points.push(Point(x, y - 1));
                    }
                    _ => unreachable!(),
                }
            }
            Direction::South => {
                y -= (dis - 1) as i64;
                match next_dir {
                    Direction::East => {
                        y -= 1;
                        right_points.push(Point(x, y));
                        x += 1;
                        left_points.push(Point(x, y + 1));
                    }
                    Direction::West => {
                        right_points.push(Point(x, y));
                        left_points.push(Point(x + 1, y - 1));
                    }
                    _ => unreachable!(),
                }
            }
            Direction::East => {
                x += (dis - 1) as i64;
                match next_dir {
                    Direction::North => {
                        x += 1;
                        right_points.push(Point(x, y));
                        y += 1;
                        left_points.push(Point(x - 1, y));
                    }
                    Direction::South => {
                        right_points.push(Point(x, y));
                        left_points.push(Point(x + 1, y + 1));
                    }
                    _ => unreachable!(),
                }
            }
            Direction::West => {
                x -= (dis - 1) as i64;
                match next_dir {
                    Direction::North => {
                        right_points.push(Point(x, y));
                        left_points.push(Point(x - 1, y - 1));
                    }
                    Direction::South => {
                        x -= 1;
                        right_points.push(Point(x, y));
                        y -= 1;
                        left_points.push(Point(x + 1, y));
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    let a1: i64 = once(right_points.last().unwrap())
        .chain(right_points.iter())
        .zip(right_points.iter())
        .map(|(p1, p2)| p1.cross(p2))
        .sum();
    let a2: i64 = once(left_points.last().unwrap())
        .chain(left_points.iter())
        .zip(left_points.iter())
        .map(|(p1, p2)| p1.cross(p2))
        .sum();
    a1.abs().max(a2.abs()) / 2
}

pub fn part1(input: &str) -> i64 {
    let input = parse_part1(input);
    solve(&input)
}

pub fn part2(input: &str) -> i64 {
    let input = parse_part2(input);
    solve(&input)
}

crate::samples! {
    (part1_sample, part1, "sample.in", "62"),
    (part1_puzzle, part1, "puzzle.in", "45159"),
    (part2_sample, part2, "sample.in", "952408144115"),
    (part2_puzzle, part2, "puzzle.in", "134549294799713"),
}
