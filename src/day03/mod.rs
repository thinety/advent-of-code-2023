mod parse;
use parse::parse;

struct Node {
    r#type: NodeType,
    neighbors: Vec<usize>,
}

enum NodeType {
    Number(u32),
    Char(char),
}

pub fn part1(input: &str) -> u32 {
    let nodes = parse(input);

    nodes
        .iter()
        .filter_map(|node| {
            if let NodeType::Number(n) = node.r#type {
                for &ni in &node.neighbors {
                    if let NodeType::Char(c) = nodes[ni].r#type {
                        if c != '.' {
                            return Some(n);
                        }
                    }
                }
            };
            None
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let nodes = parse(input);

    nodes
        .iter()
        .filter_map(|node| {
            if let NodeType::Char('*') = node.r#type {
                let mut part_numbers = 0;
                let mut gear_ratio = 1;
                for &ni in &node.neighbors {
                    if let NodeType::Number(n) = nodes[ni].r#type {
                        part_numbers += 1;
                        gear_ratio *= n;
                    }
                }
                if part_numbers == 2 {
                    return Some(gear_ratio);
                }
            }
            None
        })
        .sum()
}

crate::samples! {
    (part1, part1_sample, "sample.in", "sample.out1"),
    (part1, part1_puzzle, "puzzle.in", "puzzle.out1"),
    (part2, part2_sample, "sample.in", "sample.out2"),
    (part2, part2_puzzle, "puzzle.in", "puzzle.out2"),
}
