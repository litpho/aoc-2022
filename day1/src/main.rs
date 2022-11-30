use anyhow::Result;
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

    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {}", result);
    println!("Time spent: {}", took);

    Ok(())
}

fn part_one(_input: &[u16]) -> usize {
    unimplemented!()
}

fn part_two(_input: &[u16]) -> usize {
    unimplemented!()
}

fn parse(input: &str) -> IResult<&str, Vec<u16>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, u16> {
    map(digit1, |num: &str| num.parse::<u16>().unwrap())(input)
}

fn read_input() -> Result<Vec<u16>> {
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

        assert_eq!(0, count);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input()?;

        let count = part_two(&input);

        assert_eq!(0, count);

        Ok(())
    }
}
