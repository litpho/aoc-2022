use std::{cmp::Ordering, collections::HashSet};

use anyhow::{Error, Result};
use nom::{
    character::{complete, complete::line_ending, complete::one_of},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {}", took);
    let input = result?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {}", took);

    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[Instruction]) -> usize {
    run_instructions(input, 2)
}

fn part_two(input: &[Instruction]) -> usize {
    run_instructions(input, 10)
}

fn run_instructions(instructions: &[Instruction], size: usize) -> usize {
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut body: Vec<(isize, isize)> = vec![(0, 0); size];
    visited.insert(*body.last().unwrap());

    for instruction in instructions.iter() {
        match instruction {
            Instruction::Up(steps) => {
                for _ in 0..*steps {
                    body[0].1 += 1;
                    visited.insert(move_body(&mut body));
                }
            }
            Instruction::Right(steps) => {
                for _ in 0..*steps {
                    body[0].0 += 1;
                    visited.insert(move_body(&mut body));
                }
            }
            Instruction::Down(steps) => {
                for _ in 0..*steps {
                    body[0].1 -= 1;
                    visited.insert(move_body(&mut body));
                }
            }
            Instruction::Left(steps) => {
                for _ in 0..*steps {
                    body[0].0 -= 1;
                    visited.insert(move_body(&mut body));
                }
            }
        }
    }

    visited.len()
}

fn move_body(body: &mut [(isize, isize)]) -> (isize, isize) {
    for i in 0..body.len() - 1 {
        body[i + 1] = move_segment(body[i + 1], &body[i]);
    }
    *body.last().unwrap()
}

fn move_segment(mut next: (isize, isize), previous: &(isize, isize)) -> (isize, isize) {
    if previous.0.abs_diff(next.0) <= 1 && previous.1.abs_diff(next.1) <= 1 {
        return next;
    }

    match previous.0.cmp(&next.0) {
        Ordering::Greater => next.0 += 1,
        Ordering::Less => next.0 -= 1,
        Ordering::Equal => {}
    }

    match previous.1.cmp(&next.1) {
        Ordering::Greater => next.1 += 1,
        Ordering::Less => next.1 -= 1,
        Ordering::Equal => {}
    }

    next
}

#[derive(Debug)]
enum Instruction {
    Up(u8),
    Right(u8),
    Down(u8),
    Left(u8),
}

impl TryFrom<(char, u8)> for Instruction {
    type Error = Error;

    fn try_from(value: (char, u8)) -> std::result::Result<Self, Self::Error> {
        let (direction, steps) = value;
        match direction {
            'U' => Ok(Instruction::Up(steps)),
            'R' => Ok(Instruction::Right(steps)),
            'D' => Ok(Instruction::Down(steps)),
            'L' => Ok(Instruction::Left(steps)),
            _ => Err(Error::msg(format!("{direction} is not a valid direction"))),
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Instruction> {
    map_res(
        separated_pair(one_of("URDL"), complete::char(' '), complete::u8),
        Instruction::try_from,
    )(input)
}

fn parse_input(input: &'static str) -> Result<Vec<Instruction>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");
    const TESTDATA2: &str = include_str!("test2.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(13, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let input = parse_input(DATA)?;
        assert_eq!(6486, part_one(&input));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(1, part_two(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata2() -> Result<()> {
        assert_eq!(36, part_two(&parse_input(TESTDATA2)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = parse_input(DATA)?;
        assert_eq!(2678, part_two(&input));

        Ok(())
    }
}
