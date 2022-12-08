use std::collections::HashSet;

use anyhow::Result;
use nom::{
    character::{complete::alpha1, complete::line_ending},
    multi::separated_list1,
    IResult,
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

fn part_one(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|s| s.split_at(s.len() / 2))
        .map(|(a, b)| get_sum(a, &[b]))
        .sum::<u32>()
}

fn part_two(input: &[&str]) -> u32 {
    input
        .chunks(3)
        .map(|lines| get_sum(lines[0], &(lines[1..=2])))
        .sum::<u32>()
}

fn get_sum(a: &str, rest: &[&str]) -> u32 {
    a.chars()
        .collect::<HashSet<char>>()
        .iter()
        .filter(|a_char| rest.iter().all(|r| r.contains(**a_char)))
        .map(|c| get_value(*c))
        .sum::<u32>()
}

fn get_value(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => panic!("Invalid character"),
    }
}

fn parse(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

fn parse_input(input: &'static str) -> Result<Vec<&'static str>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(157, part_one(&parse(TESTDATA)?.1));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let input = parse_input(DATA)?;
        assert_eq!(7980, part_one(&input));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(70, part_two(&parse(TESTDATA)?.1));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = parse_input(DATA)?;
        assert_eq!(2881, part_two(&input));

        Ok(())
    }
}
