use nom::bytes::complete::{tag, take_until1};
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

pub(super) fn parse(input: &str) -> Vec<(Vec<u8>, Vec<usize>)> {
    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, str::parse)(input)
    }
    fn numbers(input: &str) -> IResult<&str, Vec<usize>> {
        separated_list1(tag(","), number)(input)
    }
    fn line(input: &str) -> IResult<&str, (Vec<u8>, Vec<usize>)> {
        map(
            separated_pair(take_until1(" "), tag(" "), numbers),
            |(springs, nums)| (springs.as_bytes().to_vec(), nums),
        )(input)
    }
    fn parse(input: &str) -> IResult<&str, Vec<(Vec<u8>, Vec<usize>)>> {
        separated_list1(tag("\n"), line)(input)
    }

    let (rest, output) = parse(input.trim_end()).unwrap();
    assert!(rest.is_empty());
    output
}
