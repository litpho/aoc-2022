use std::collections::HashSet;

use anyhow::Result;
use nom::{
    character::{complete::alpha1, complete::line_ending},
    multi::separated_list1,
    IResult,
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

fn read_input() -> Result<Vec<&'static str>> {
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

        assert_eq!(7980, count);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input()?;

        let count = part_two(&input);

        assert_eq!(2881, count);

        Ok(())
    }
}
