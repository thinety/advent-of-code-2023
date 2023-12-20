mod parse;
use parse::parse;

use std::collections::HashMap;
use std::iter::{once, zip};

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

fn is_part_good(workflows: &Workflows, part: &Part) -> bool {
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
}

pub fn part1(input: &str) -> u32 {
    let (workflows, parts) = parse(input);

    parts
        .iter()
        .filter(|part| is_part_good(&workflows, part))
        .map(|part| part.x + part.m + part.a + part.s)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let (workflows, _) = parse(input);

    let mut x_relevant_values = vec![];
    let mut m_relevant_values = vec![];
    let mut a_relevant_values = vec![];
    let mut s_relevant_values = vec![];

    for workflow in workflows.values() {
        for rule in &workflow.rules {
            let (rating, op, value) = rule.condition;
            let value = match op {
                Op::GreaterThan => value + 1,
                Op::LesserThan => value,
            };
            let rating_relevant_values = match rating {
                Rating::X => &mut x_relevant_values,
                Rating::M => &mut m_relevant_values,
                Rating::A => &mut a_relevant_values,
                Rating::S => &mut s_relevant_values,
            };
            rating_relevant_values.push(value);
        }
    }

    x_relevant_values.sort();
    m_relevant_values.sort();
    a_relevant_values.sort();
    s_relevant_values.sort();

    let mut ans = 0;
    for (&x, next_x) in zip(
        once(&1).chain(x_relevant_values.iter()),
        x_relevant_values.iter().chain(once(&4001)),
    ) {
        for (&m, next_m) in zip(
            once(&1).chain(m_relevant_values.iter()),
            m_relevant_values.iter().chain(once(&4001)),
        ) {
            for (&a, next_a) in zip(
                once(&1).chain(a_relevant_values.iter()),
                a_relevant_values.iter().chain(once(&4001)),
            ) {
                for (&s, next_s) in zip(
                    once(&1).chain(s_relevant_values.iter()),
                    s_relevant_values.iter().chain(once(&4001)),
                ) {
                    let part = Part { x, m, a, s };
                    if is_part_good(&workflows, &part) {
                        ans += ((next_x - x) as u64)
                            * ((next_m - m) as u64)
                            * ((next_a - a) as u64)
                            * ((next_s - s) as u64);
                    }
                }
            }
        }
    }
    ans
}

crate::samples! {
    (part1_sample, part1, "sample.in", 19114),
    (part1_puzzle, part1, "puzzle.in", 333263),
    (part2_sample, part2, "sample.in", 167409079868000),
    (part2_puzzle, part2, "puzzle.in", 130745440937650),
}
