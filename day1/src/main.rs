use anyhow::Result;
use nom::sequence::pair;
use nom::{
    character::complete::{digit1, line_ending},
    combinator::map,
    multi::separated_list1,
    IResult,
};
use std::{fs, io::Read};

fn main() -> Result<()> {
    let input = read_input()?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {}", result);
    println!("Time spent: {}", took);

    let (took, result) = took::took(|| part_two(input));
    println!("Result part two: {}", result);
    println!("Time spent: {}", took);

    Ok(())
}

fn part_one(input: &[u32]) -> u32 {
    *input.iter().max().unwrap()
}

fn part_two(mut input: Vec<u32>) -> u32 {
    input.sort_by(|a, b| b.cmp(a));
    input[0..3].iter().sum()
}

fn parse(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(pair(line_ending, line_ending), parse_lines)(input)
}

fn parse_lines(input: &str) -> IResult<&str, u32> {
    map(separated_list1(line_ending, parse_line), |items| {
        items.iter().sum()
    })(input)
}

fn parse_line(input: &str) -> IResult<&str, u32> {
    map(digit1, |num: &str| num.parse::<u32>().unwrap())(input)
}

fn read_input() -> Result<Vec<u32>> {
    let mut buf = String::new();
    fs::File::open("src/input.txt")?.read_to_string(&mut buf)?;

    let (_, input) = parse(&buf).ok().unwrap();

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let input = read_input()?;

        let count = part_one(&input);

        assert_eq!(72511, count);

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
