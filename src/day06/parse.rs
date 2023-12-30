use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace0, space1};
use nom::combinator::{all_consuming, map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{pair, preceded, separated_pair, terminated};
use nom::IResult;

pub(super) fn parse_part1(input: &str) -> Vec<(u64, u64)> {
    fn number(input: &str) -> IResult<&str, u64> {
        map_res(digit1, str::parse)(input)
    }
    fn numbers(input: &str) -> IResult<&str, Vec<u64>> {
        separated_list1(space1, number)(input)
    }
    fn parse(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
        map(
            separated_pair(
                preceded(pair(tag("Time:"), space1), numbers),
                tag("\n"),
                preceded(pair(tag("Distance:"), space1), numbers),
            ),
            |(times, distances)| {
                times
                    .iter()
                    .copied()
                    .zip(distances.iter().copied())
                    .collect()
            },
        )(input)
    }

    let (_, output) = all_consuming(terminated(parse, multispace0))(input).unwrap();
    output
}

pub(super) fn parse_part2(input: &str) -> (u64, u64) {
    fn number(input: &str) -> IResult<&str, u64> {
        map_res(separated_list1(space1, digit1), |digits| {
            digits.join("").parse()
        })(input)
    }
    fn parse(input: &str) -> IResult<&str, (u64, u64)> {
        separated_pair(
            preceded(pair(tag("Time:"), space1), number),
            tag("\n"),
            preceded(pair(tag("Distance:"), space1), number),
        )(input)
    }

    let (_, output) = all_consuming(terminated(parse, multispace0))(input).unwrap();
    output
}
