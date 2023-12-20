use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::{map, map_res, value};
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded, separated_pair, tuple};
use nom::IResult;

use super::*;

pub(super) fn parse(input: &str) -> (Workflows, Vec<Part>) {
    fn number(input: &str) -> IResult<&str, u32> {
        map_res(digit1, str::parse)(input)
    }

    fn rating(input: &str) -> IResult<&str, Rating> {
        alt((
            value(Rating::X, tag("x")),
            value(Rating::M, tag("m")),
            value(Rating::A, tag("a")),
            value(Rating::S, tag("s")),
        ))(input)
    }
    fn op(input: &str) -> IResult<&str, Op> {
        alt((
            value(Op::LessThan, tag("<")),
            value(Op::GreaterThan, tag(">")),
        ))(input)
    }
    fn rule(input: &str) -> IResult<&str, Rule> {
        map(
            separated_pair(tuple((rating, op, number)), tag(":"), alpha1),
            |(condition, workflow)| Rule {
                condition,
                workflow: workflow.to_string(),
            },
        )(input)
    }
    fn workflow(input: &str) -> IResult<&str, Workflow> {
        map(
            delimited(
                tag("{"),
                separated_pair(separated_list1(tag(","), rule), tag(","), alpha1),
                tag("}"),
            ),
            |(rules, default_workflow)| Workflow {
                rules,
                default_workflow: default_workflow.to_string(),
            },
        )(input)
    }
    fn workflows(input: &str) -> IResult<&str, Workflows> {
        map(
            separated_list1(tag("\n"), pair(alpha1, workflow)),
            |workflows| {
                workflows
                    .into_iter()
                    .map(|(name, workflow)| (name.to_string(), workflow))
                    .collect()
            },
        )(input)
    }

    fn part(input: &str) -> IResult<&str, Part> {
        map(
            delimited(
                tag("{"),
                tuple((
                    preceded(tag("x="), number),
                    preceded(tag(",m="), number),
                    preceded(tag(",a="), number),
                    preceded(tag(",s="), number),
                )),
                tag("}"),
            ),
            |(x, m, a, s)| [x, m, a, s],
        )(input)
    }

    fn parse(input: &str) -> IResult<&str, (Workflows, Vec<Part>)> {
        separated_pair(workflows, tag("\n\n"), separated_list1(tag("\n"), part))(input)
    }

    let (input, output) = parse(input.trim_end()).unwrap();
    assert!(input.is_empty());
    output
}
