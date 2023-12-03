use anyhow::{Error, Result};
use nom::{
    character::complete::{self, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::pair,
    IResult,
};

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {}", took);
    let input = result?;

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
    input[..3].iter().sum()
}

fn parse(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(pair(line_ending, line_ending), parse_lines)(input)
}

fn parse_lines(input: &str) -> IResult<&str, u32> {
    map(separated_list1(line_ending, complete::u32), |items| {
        items.iter().sum()
    })(input)
}

fn parse_input(input: &'static str) -> Result<Vec<u32>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(&24000, part_one(&parse(TESTDATA)?.1)?);

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let input = parse_input(DATA)?;
        assert_eq!(&72511, part_one(&input)?);

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(45000, part_two(parse(TESTDATA)?.1));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = parse_input(DATA)?;
        assert_eq!(212117, part_two(input));

        Ok(())
    }
}
