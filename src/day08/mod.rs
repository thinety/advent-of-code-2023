mod parse;
use parse::parse;

struct Node {
    left: String,
    right: String,
}

#[derive(Clone)]
enum Direction {
    Left,
    Right,
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut old_r = a;
    let mut r = b;

    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
    }

    old_r
}

fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

pub fn part1(input: &str) -> u64 {
    let (directions, nodes) = parse(input);

    let mut current = "AAA";
    let mut directions = directions.iter().cycle();
    let mut total = 0;

    while current != "ZZZ" {
        match directions.next().unwrap() {
            Direction::Left => {
                current = &nodes[current].left;
            }
            Direction::Right => {
                current = &nodes[current].right;
            }
        }
        total += 1;
    }

    total
}

pub fn part2(input: &str) -> u64 {
    let (directions, nodes) = parse(input);

    nodes
        .keys()
        .filter_map(|k| k.ends_with('A').then_some(k.as_ref()))
        .map(|mut current: &str| {
            let mut directions = directions.iter().cycle();
            let mut total = 0;

            while !current.ends_with('Z') {
                match directions.next().unwrap() {
                    Direction::Left => {
                        current = &nodes[current].left;
                    }
                    Direction::Right => {
                        current = &nodes[current].right;
                    }
                }
                total += 1;
            }

            total
        })
        .fold(1, lcm)
}

crate::samples! {
    (part1_sample1, part1, "sample1.in", "2"),
    (part1_sample2, part1, "sample2.in", "6"),
    (part1_puzzle, part1, "puzzle.in", "21883"),
    (part2_sample3, part2, "sample3.in", "6"),
    (part2_puzzle, part2, "puzzle.in", "12833235391111"),
}
