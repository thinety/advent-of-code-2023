mod parse;
use parse::parse;

struct Node {
    r#type: NodeType,
    neighbors: Vec<usize>,
}

enum NodeType {
    Number(u32),
    Char(u8),
}

pub fn part1(input: &str) -> u32 {
    let nodes = parse(input);

    nodes
        .iter()
        .filter_map(|node| {
            if let NodeType::Number(n) = node.r#type {
                for &ni in &node.neighbors {
                    if let NodeType::Char(c) = nodes[ni].r#type {
                        if c != b'.' {
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
            if let NodeType::Char(b'*') = node.r#type {
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
    (part1_sample, part1, "sample.in", "4361"),
    (part1_puzzle, part1, "puzzle.in", "520135"),
    (part2_sample, part2, "sample.in", "467835"),
    (part2_puzzle, part2, "puzzle.in", "72514855"),
}
