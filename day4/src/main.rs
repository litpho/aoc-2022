use anyhow::Result;
use nom::character::complete;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::sequence::separated_pair;
use nom::{character::complete::line_ending, multi::separated_list1, IResult};
use std::ops::RangeInclusive;
use std::str::FromStr;

fn main() -> Result<()> {
    let input = read_input()?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {}", result);
    println!("Time spent: {took}");

    let input = read_input()?;
    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {}", result);
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[AssignmentPair]) -> usize {
    input
        .iter()
        .filter(|ap| ap.first.envelopes(&ap.second) || ap.second.envelopes(&ap.first))
        .count()
}

fn part_two(input: &[AssignmentPair]) -> usize {
    input
        .iter()
        .filter(|ap| ap.first.overlaps(&ap.second) || ap.second.overlaps(&ap.first))
        .count()
}

trait RangeInclusiveExt {
    fn envelopes(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

impl RangeInclusiveExt for RangeInclusive<u32> {
    fn envelopes(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.start() <= other.start() && self.end() >= other.start())
            || (self.start() <= other.end() && self.end() >= other.end())
    }
}

struct AssignmentPair {
    first: RangeInclusive<u32>,
    second: RangeInclusive<u32>,
}

fn parse(input: &str) -> IResult<&str, Vec<AssignmentPair>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, AssignmentPair> {
    map(
        separated_pair(parse_range, complete::char(','), parse_range),
        |(first, second)| AssignmentPair { first, second },
    )(input)
}

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    map(
        separated_pair(parse_item, complete::char('-'), parse_item),
        |(start, end)| start..=end,
    )(input)
}

fn parse_item(input: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str)(input)
}

fn read_input() -> Result<Vec<AssignmentPair>> {
    let buf = include_str!("input.txt");

    let (_, input) = parse(buf).expect("Parse failure");

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let input = read_input()?;

        let count = part_one(&input);

        assert_eq!(515, count);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input()?;

        let count = part_two(&input);

        assert_eq!(883, count);

        Ok(())
    }
}
