use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, one_of, space0, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (crates, instructions) = parse_input(DATA)?;

    let (took, result) = took::took(|| part_one(crates, &instructions));
    println!("Result part one: {}", result);
    println!("Time spent: {}", took);

    let (crates, instructions) = parse_input(DATA)?;

    let (took, result) = took::took(|| part_two(crates, &instructions));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(mut crates: Vec<Vec<char>>, instructions: &[Instruction]) -> String {
    for instruction in instructions.iter() {
        (0..instruction.amount).for_each(|_| {
            let cr = crates[instruction.from - 1].pop().unwrap();
            crates[instruction.to - 1].push(cr);
        });
    }

    crates
        .iter()
        .map(|cr| cr.last().unwrap())
        .collect::<String>()
}

fn part_two(mut crates: Vec<Vec<char>>, instructions: &[Instruction]) -> String {
    for instruction in instructions.iter() {
        let idx = crates[instruction.from - 1].len() - instruction.amount;
        let buf = crates[instruction.from - 1].split_off(idx);
        crates[instruction.to - 1].extend(buf);
    }

    crates
        .iter()
        .map(|cr| cr.last().unwrap())
        .collect::<String>()
}

#[derive(Debug)]
struct Instruction {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}

impl From<(u8, u8, u8)> for Instruction {
    fn from(tuple: (u8, u8, u8)) -> Self {
        Instruction {
            amount: tuple.0 as usize,
            from: tuple.1 as usize,
            to: tuple.2 as usize,
        }
    }
}

fn parse(input: &str) -> IResult<&str, (Vec<Vec<char>>, Vec<Instruction>)> {
    separated_pair(
        parse_crates,
        pair(line_ending, line_ending),
        parse_instructions,
    )(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_instruction)(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), complete::u8),
            preceded(tag(" from "), complete::u8),
            preceded(tag(" to "), complete::u8),
        )),
        Instruction::from,
    )(input)
}

fn parse_crates(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    map(parse_crate_lines, transpose)(input)
}

fn transpose(lines: Vec<Vec<Option<char>>>) -> Vec<Vec<char>> {
    let mut output = (0..lines.last().unwrap().len())
        .map(|_| vec![])
        .collect::<Vec<Vec<char>>>();
    lines.iter().rev().for_each(|line| {
        line.iter().enumerate().for_each(|(i, cr)| {
            if let Some(c) = cr {
                output[i].push(*c);
            }
        })
    });
    output
}

fn parse_crate_lines(input: &str) -> IResult<&str, Vec<Vec<Option<char>>>> {
    terminated(
        separated_list1(line_ending, parse_crate_line),
        pair(line_ending, parse_index_line),
    )(input)
}

fn parse_index_line(input: &str) -> IResult<&str, Vec<u8>> {
    preceded(space0, separated_list1(space1, complete::u8))(input)
}

fn parse_crate_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list1(
        complete::char(' '),
        map(alt((parse_crate, parse_empty_crate)), |a| {
            if a == ' ' {
                None
            } else {
                Some(a)
            }
        }),
    )(input)
}

fn parse_empty_crate(input: &str) -> IResult<&str, char> {
    delimited(
        complete::char(' '),
        complete::char(' '),
        complete::char(' '),
    )(input)
}

fn parse_crate(input: &str) -> IResult<&str, char> {
    delimited(
        complete::char('['),
        one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
        complete::char(']'),
    )(input)
}

fn parse_input(input: &'static str) -> Result<(Vec<Vec<char>>, Vec<Instruction>)> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        let (crates, instructions) = parse(TESTDATA)?.1;
        assert_eq!("CMZ", part_one(crates, &instructions));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let (crates, instructions) = parse_input(DATA)?;
        assert_eq!("FJSRQCFTN", part_one(crates, &instructions));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        let (crates, instructions) = parse(TESTDATA)?.1;
        assert_eq!("MCD", part_two(crates, &instructions));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let (crates, instructions) = parse_input(DATA)?;
        assert_eq!("CJVLJQPHS", part_two(crates, &instructions));

        Ok(())
    }
}
