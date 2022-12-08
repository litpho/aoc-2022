use std::ops::RangeInclusive;

use anyhow::Result;
use nom::{
    character::complete, character::complete::line_ending, combinator::map, multi::separated_list1,
    sequence::separated_pair, IResult,
};

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {}", took);
    let input = result?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {}", result);
    println!("Time spent: {took}");

    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {}", took);
    let input = result?;

    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {}", result);
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[AssignmentPair]) -> usize {
    input
        .iter()
        .filter(|ap| ap.one_range_envelopes_the_other())
        .count()
}

fn part_two(input: &[AssignmentPair]) -> usize {
    input
        .iter()
        .filter(|ap| ap.one_range_overlaps_the_other())
        .count()
}

struct AssignmentPair {
    first: RangeInclusive<u32>,
    second: RangeInclusive<u32>,
}

impl AssignmentPair {
    pub fn one_range_envelopes_the_other(&self) -> bool {
        (self.first.contains(self.second.start()) && self.first.contains(self.second.end()))
            || (self.second.contains(self.first.start()) && self.second.contains(self.first.end()))
    }

    pub fn one_range_overlaps_the_other(&self) -> bool {
        !(self.first.start() > self.second.end() || self.first.end() < self.second.start())
    }
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
        separated_pair(complete::u32, complete::char('-'), complete::u32),
        |(start, end)| start..=end,
    )(input)
}

fn parse_input(input: &'static str) -> Result<Vec<AssignmentPair>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(2, part_one(&parse(TESTDATA)?.1));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let input = parse_input(DATA)?;
        assert_eq!(515, part_one(&input));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(4, part_two(&parse(TESTDATA)?.1));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = parse_input(DATA)?;
        assert_eq!(883, part_two(&input));

        Ok(())
    }
}
