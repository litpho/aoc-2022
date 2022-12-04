use std::ops::RangeInclusive;

use anyhow::Result;
use nom::{
    character::complete, character::complete::line_ending, combinator::map, multi::separated_list1,
    sequence::separated_pair, IResult,
};

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