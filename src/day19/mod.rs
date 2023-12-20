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
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}
#[derive(Clone, Copy)]
enum Op {
    LessThan,
    GreaterThan,
}

type Part = [u32; 4];

pub fn part1(input: &str) -> u32 {
    let (workflows, parts) = parse(input);

    parts
        .iter()
        .filter(|part| {
            let mut name = "in";
            loop {
                if name == "A" {
                    return true;
                }
                if name == "R" {
                    return false;
                }

                let workflow = &workflows[name];

                let mut next: Option<&str> = None;
                for rule in &workflow.rules {
                    let (rating, op, value) = rule.condition;
                    let ok = match op {
                        Op::LessThan => part[rating as usize] < value,
                        Op::GreaterThan => part[rating as usize] > value,
                    };
                    if ok {
                        next = Some(&rule.workflow);
                        break;
                    }
                }
                name = next.unwrap_or(&workflow.default_workflow);
            }
        })
        .map(|part| part[0] + part[1] + part[2] + part[3])
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let (workflows, _) = parse(input);

    let mut parts = Vec::new();
    parts.push(("in", [1, 1, 1, 1], [4001, 4001, 4001, 4001]));

    let mut ans = 0;
    while let Some((mut name, mut lower_part, mut upper_part)) = parts.pop() {
        loop {
            if name == "A" {
                ans += ((upper_part[0] - lower_part[0]) as u64)
                    * ((upper_part[1] - lower_part[1]) as u64)
                    * ((upper_part[2] - lower_part[2]) as u64)
                    * ((upper_part[3] - lower_part[3]) as u64);
                break;
            }
            if name == "R" {
                break;
            }

            let workflow = &workflows[name];

            let mut next: Option<&str> = None;
            for rule in &workflow.rules {
                let (rating, op, value) = rule.condition;
                match op {
                    Op::LessThan => {
                        if upper_part[rating as usize] - 1 < value {
                            next = Some(&rule.workflow);
                            break;
                        }
                        if lower_part[rating as usize] < value {
                            let new_lower_part = lower_part;

                            let mut new_upper_part = upper_part;
                            new_upper_part[rating as usize] = value;

                            lower_part[rating as usize] = value;

                            parts.push((&rule.workflow, new_lower_part, new_upper_part));
                        }
                    }
                    Op::GreaterThan => {
                        if value < lower_part[rating as usize] {
                            next = Some(&rule.workflow);
                            break;
                        }
                        if value < upper_part[rating as usize] - 1 {
                            let mut new_lower_part = lower_part;
                            new_lower_part[rating as usize] = value + 1;

                            let new_upper_part = upper_part;

                            upper_part[rating as usize] = value + 1;

                            parts.push((&rule.workflow, new_lower_part, new_upper_part));
                        }
                    }
                }
            }
            name = next.unwrap_or(&workflow.default_workflow);
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
