mod parse;
use parse::parse;

use std::collections::HashMap;

type Workflows = HashMap<String, Workflow>;
struct Workflow {
    rules: Vec<Rule>,
    default_workflow: String,
}
struct Rule {
    condition: (Rating, Op, u32),
    workflow: String,
}
#[derive(Clone, Copy)]
enum Rating {
    X,
    M,
    A,
    S,
}
#[derive(Clone, Copy)]
enum Op {
    GreaterThan,
    LesserThan,
}

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

pub fn part1(input: &str) -> u32 {
    let (workflows, parts) = parse(input);

    parts
        .iter()
        .filter(|part| {
            let mut name = "in";
            loop {
                let workflow = &workflows[name];

                let mut next: Option<&str> = None;
                for rule in &workflow.rules {
                    let (rating, op, value) = rule.condition;
                    let rating = match rating {
                        Rating::X => part.x,
                        Rating::M => part.m,
                        Rating::A => part.a,
                        Rating::S => part.s,
                    };
                    let ok = match op {
                        Op::GreaterThan => rating > value,
                        Op::LesserThan => rating < value,
                    };
                    if ok {
                        next = Some(&rule.workflow);
                        break;
                    }
                }
                let next = next.unwrap_or(&workflow.default_workflow);

                if next == "A" {
                    return true;
                }
                if next == "R" {
                    return false;
                }
                name = next;
            }
        })
        .map(|part| part.x + part.m + part.a + part.s)
        .sum()
}

crate::samples! {
    (part1_sample, part1, "sample.in", 19114),
    (part1_puzzle, part1, "puzzle.in", 333263),
}
