use anyhow::{Error, Result};
use nom::{
    character::complete::{self, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::pair,
    IResult,
};

fn main() -> Result<()> {
    let input = read_input()?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {}", result?);
    println!("Time spent: {}", took);

    let (took, result) = took::took(|| part_two(input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[u32]) -> Result<&u32> {
    input
        .iter()
        .max()
        .ok_or_else(|| Error::msg("There was no maximum"))
}

fn part_two(mut input: Vec<u32>) -> u32 {
    input.sort_by(|a, b| b.cmp(a));
    input[0..3].iter().sum()
}

fn parse(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(pair(line_ending, line_ending), parse_lines)(input)
}

fn parse_lines(input: &str) -> IResult<&str, u32> {
    map(separated_list1(line_ending, complete::u32), |items| {
        items.iter().sum()
    })(input)
}

fn read_input() -> Result<Vec<u32>> {
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

        let count = part_one(&input)?;

        assert_eq!(72511, *count);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input()?;

        let count = part_two(input);

        assert_eq!(212117, count);

        Ok(())
    }
}
