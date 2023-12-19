mod parse;
use parse::{parse_part1, parse_part2};

#[derive(Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn solve(input: &[(Direction, u32)]) -> i64 {
    let mut boundary = 0;

    let mut points = Vec::new();

    let (mut x, mut y) = (0, 0);
    points.push((x, y));

    for (dir, dis) in input {
        let dis = *dis as i64;
        boundary += dis;

        match dir {
            Direction::North => {
                y += dis;
            }
            Direction::South => {
                y -= dis;
            }
            Direction::East => {
                x += dis;
            }
            Direction::West => {
                x -= dis;
            }
        }
        points.push((x, y));
    }

    // https://en.wikipedia.org/wiki/Shoelace_formula
    // can be derived from Stokes' theorem
    let area = points
        .iter()
        .zip(points.iter().skip(1))
        .map(|((x1, y1), (x2, y2))| x1 * y2 - x2 * y1)
        .sum::<i64>()
        .abs()
        / 2;

    // https://en.wikipedia.org/wiki/Pick's_theorem
    let inside = area - boundary / 2 + 1;

    inside + boundary
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
