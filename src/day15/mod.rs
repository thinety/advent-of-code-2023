mod parse;
use parse::parse;

use std::collections::HashMap;

struct Step {
    label: String,
    operation: Operation,
}

#[derive(Clone)]
enum Operation {
    Dash,
    Equal(u32),
}

#[derive(Clone)]
struct Box {
    lenses: HashMap<String, Lens>,
    first: Option<String>,
    last: Option<String>,
}
#[derive(Clone)]
struct Lens {
    focal_length: u32,
    previous: Option<String>,
    next: Option<String>,
}

fn hash(input: &str) -> u32 {
    input.chars().fold(0, |mut x, c| {
        x += c as u32;
        x *= 17;
        x % 256
    })
}

pub fn part1(input: &str) -> u32 {
    input.trim_end().split(',').map(hash).sum()
}

pub fn part2(input: &str) -> u32 {
    let input = parse(input);

    let mut boxes: Vec<Box> = vec![
        Box {
            lenses: HashMap::new(),
            first: None,
            last: None,
        };
        256
    ];
    for step in &input {
        let r#box = &mut boxes[hash(&step.label) as usize];
        match step.operation {
            Operation::Dash => {
                if let Some(lens) = r#box.lenses.remove(&step.label) {
                    match (lens.previous, lens.next) {
                        (Some(previous), Some(next)) => {
                            r#box.lenses.get_mut(&previous).unwrap().next = Some(next.clone());
                            r#box.lenses.get_mut(&next).unwrap().previous = Some(previous.clone());
                        }
                        (Some(previous), None) => {
                            r#box.lenses.get_mut(&previous).unwrap().next = None;
                            r#box.last = Some(previous);
                        }
                        (None, Some(next)) => {
                            r#box.lenses.get_mut(&next).unwrap().previous = None;
                            r#box.first = Some(next);
                        }
                        (None, None) => {
                            r#box.first = None;
                            r#box.last = None;
                        }
                    }
                }
            }
            Operation::Equal(n) => match r#box.lenses.get_mut(&step.label) {
                Some(lens) => {
                    lens.focal_length = n;
                }
                None => {
                    match &r#box.last {
                        Some(last) => {
                            r#box.lenses.get_mut(last).unwrap().next = Some(step.label.clone());
                        }
                        None => {
                            r#box.first = Some(step.label.clone());
                        }
                    }
                    r#box.lenses.insert(
                        step.label.clone(),
                        Lens {
                            focal_length: n,
                            previous: r#box.last.take(),
                            next: None,
                        },
                    );
                    r#box.last = Some(step.label.clone());
                }
            },
        }
    }

    let mut ans = 0;
    for (b, r#box) in boxes.iter().enumerate() {
        let mut i = 0;
        let mut next = &r#box.first;
        loop {
            let lens = r#box
                .lenses
                .get(match next {
                    Some(next) => next,
                    None => break,
                })
                .unwrap();
            ans += ((b as u32) + 1) * (i + 1) * lens.focal_length;
            i += 1;
            next = &lens.next;
        }
    }
    ans
}

crate::samples! {
    (part1_sample, part1, "sample.in", 1320),
    (part1_puzzle, part1, "puzzle.in", 512797),
    (part2_sample, part2, "sample.in", 145),
    (part2_puzzle, part2, "puzzle.in", 262454),
}
