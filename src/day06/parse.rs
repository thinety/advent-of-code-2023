use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair},
    IResult,
};

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

    let (input, output) = parse(input.trim_end()).unwrap();
    assert!(input.is_empty());
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

    let (input, output) = parse(input.trim_end()).unwrap();
    assert!(input.is_empty());
    output
}
