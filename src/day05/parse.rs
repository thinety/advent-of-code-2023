use nom::{
    bytes::complete::{tag, take_until1},
    character::complete::digit1,
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

pub(super) fn parse_part1(input: &str) -> (Vec<u64>, Vec<Vec<(u64, u64, u64)>>) {
    fn number(input: &str) -> IResult<&str, u64> {
        map_res(digit1, str::parse)(input)
    }
    fn range(input: &str) -> IResult<&str, (u64, u64, u64)> {
        map(
            tuple((number, tag(" "), number, tag(" "), number)),
            |(destination, _, source, _, length)| (source, source + length, destination),
        )(input)
    }
    fn ranges(input: &str) -> IResult<&str, Vec<(u64, u64, u64)>> {
        separated_list1(tag("\n"), range)(input)
    }
    fn mapping(input: &str) -> IResult<&str, Vec<(u64, u64, u64)>> {
        preceded(terminated(take_until1("\n"), tag("\n")), ranges)(input)
    }
    fn mappings(input: &str) -> IResult<&str, Vec<Vec<(u64, u64, u64)>>> {
        separated_list1(tag("\n\n"), mapping)(input)
    }
    fn seeds(input: &str) -> IResult<&str, Vec<u64>> {
        preceded(tag("seeds: "), separated_list1(tag(" "), number))(input)
    }
    fn parse(input: &str) -> IResult<&str, (Vec<u64>, Vec<Vec<(u64, u64, u64)>>)> {
        separated_pair(seeds, tag("\n\n"), mappings)(input)
    }

    let (input, (seeds, mappings)) = parse(input.trim_end()).unwrap();
    assert!(input.is_empty());
    (seeds, mappings)
}

pub(super) fn parse_part2(input: &str) -> (Vec<(u64, u64)>, Vec<Vec<(u64, u64, u64)>>) {
    fn number(input: &str) -> IResult<&str, u64> {
        map_res(digit1, str::parse)(input)
    }
    fn range(input: &str) -> IResult<&str, (u64, u64, u64)> {
        map(
            tuple((number, tag(" "), number, tag(" "), number)),
            |(destination, _, source, _, length)| (source, source + length, destination),
        )(input)
    }
    fn ranges(input: &str) -> IResult<&str, Vec<(u64, u64, u64)>> {
        separated_list1(tag("\n"), range)(input)
    }
    fn mapping(input: &str) -> IResult<&str, Vec<(u64, u64, u64)>> {
        preceded(terminated(take_until1("\n"), tag("\n")), ranges)(input)
    }
    fn mappings(input: &str) -> IResult<&str, Vec<Vec<(u64, u64, u64)>>> {
        separated_list1(tag("\n\n"), mapping)(input)
    }
    fn seed(input: &str) -> IResult<&str, (u64, u64)> {
        map(
            separated_pair(number, tag(" "), number),
            |(start, length)| (start, start + length),
        )(input)
    }
    fn seeds(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
        preceded(tag("seeds: "), separated_list1(tag(" "), seed))(input)
    }
    fn parse(input: &str) -> IResult<&str, (Vec<(u64, u64)>, Vec<Vec<(u64, u64, u64)>>)> {
        separated_pair(seeds, tag("\n\n"), mappings)(input)
    }

    let (input, (seeds, mappings)) = parse(input.trim_end()).unwrap();
    assert!(input.is_empty());
    (seeds, mappings)
}
