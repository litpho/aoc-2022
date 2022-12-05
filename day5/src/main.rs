use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::character::complete::{one_of, space0, space1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};
use nom::IResult;

fn main() -> Result<()> {
    let (crates, instructions) = read_input()?;

    let (took, result) = took::took(|| part_one(crates, &instructions));
    println!("Result part one: {}", result);
    println!("Time spent: {}", took);

    let (crates, instructions) = read_input()?;

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
        .iter_mut()
        .map(|cr| cr.pop().unwrap())
        .collect::<String>()
}

fn part_two(mut crates: Vec<Vec<char>>, instructions: &[Instruction]) -> String {
    for instruction in instructions.iter() {
        let mut buf = vec![];
        (0..instruction.amount).for_each(|_| {
            let cr = crates[instruction.from - 1].pop().unwrap();
            buf.push(cr);
        });
        buf.reverse();
        crates[instruction.to - 1].append(&mut buf);
    }

    crates
        .iter_mut()
        .map(|cr| cr.pop().unwrap())
        .collect::<String>()
}

#[derive(Debug)]
struct Instruction {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
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
            tag("move "),
            complete::u8,
            tag(" from "),
            complete::u8,
            tag(" to "),
            complete::u8,
        )),
        |(_, amount, _, from, _, to)| Instruction {
            amount: amount as usize,
            from: from as usize,
            to: to as usize,
        },
    )(input)
}

fn parse_crates(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    map(parse_crate_lines, |mut lines| {
        let mut output: Vec<Vec<char>> = vec![];
        lines.reverse();
        lines[0].iter().for_each(|_| output.push(vec![]));
        lines.iter().for_each(|line| {
            line.iter().enumerate().for_each(|(i, cr)| {
                if let Some(c) = cr {
                    output[i].push(*c);
                }
            })
        });
        output
    })(input)
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

fn read_input() -> Result<(Vec<Vec<char>>, Vec<Instruction>)> {
    let buf = include_str!("input.txt");

    let (_, input) = parse(buf).expect("Parse failure");

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let (crates, instructions) = read_input()?;

        let result = part_one(crates, &instructions);

        assert_eq!("FJSRQCFTN", result);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let (crates, instructions) = read_input()?;

        let result = part_two(crates, &instructions);

        assert_eq!("CJVLJQPHS", result);

        Ok(())
    }
}
