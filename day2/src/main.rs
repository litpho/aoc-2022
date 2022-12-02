use anyhow::{Error, Result};
use nom::{
    character::{complete, complete::line_ending, complete::one_of},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{fs, io::Read};

fn main() -> Result<()> {
    let input = read_input()?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {}", result?);
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {}", result?);
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[(char, char)]) -> Result<u32> {
    Ok(input
        .iter()
        .map(|(them, us)| Ok((Move::try_from(*them)?, Move::try_from(*us)?)))
        .collect::<Result<Vec<(Move, Move)>>>()?
        .iter()
        .map(|(them, us)| Move::score(them, us))
        .sum())
}

fn part_two(input: &[(char, char)]) -> Result<u32> {
    Ok(input
        .iter()
        .map(|(them, us)| {
            let their_move = Move::try_from(*them)?;
            let matched_move = Move::match_move(&their_move, Outcome::try_from(*us)?);
            Ok((their_move, matched_move))
        })
        .collect::<Result<Vec<(Move, Move)>>>()?
        .iter()
        .map(|(them, us)| Move::score(them, us))
        .sum())
}

#[derive(Debug)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    pub fn value(&self) -> u32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        let outcome = match value {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => return Err(Error::msg(format!("{} is not a valid outcome", value))),
        };

        Ok(outcome)
    }
}

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn value(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    pub fn score(them: &Move, us: &Move) -> u32 {
        us.value() + Self::outcome(them, us).value()
    }

    pub fn match_move(them: &Move, outcome: Outcome) -> Move {
        match (them, outcome) {
            (Move::Rock, Outcome::Lose) => Move::Scissors,
            (Move::Rock, Outcome::Draw) => Move::Rock,
            (Move::Rock, Outcome::Win) => Move::Paper,
            (Move::Paper, Outcome::Lose) => Move::Rock,
            (Move::Paper, Outcome::Draw) => Move::Paper,
            (Move::Paper, Outcome::Win) => Move::Scissors,
            (Move::Scissors, Outcome::Lose) => Move::Paper,
            (Move::Scissors, Outcome::Draw) => Move::Scissors,
            (Move::Scissors, Outcome::Win) => Move::Rock,
        }
    }

    fn outcome(them: &Move, us: &Move) -> Outcome {
        match (them, us) {
            (Move::Rock, Move::Rock) => Outcome::Draw,
            (Move::Rock, Move::Paper) => Outcome::Win,
            (Move::Rock, Move::Scissors) => Outcome::Lose,
            (Move::Paper, Move::Rock) => Outcome::Lose,
            (Move::Paper, Move::Paper) => Outcome::Draw,
            (Move::Paper, Move::Scissors) => Outcome::Win,
            (Move::Scissors, Move::Rock) => Outcome::Win,
            (Move::Scissors, Move::Paper) => Outcome::Lose,
            (Move::Scissors, Move::Scissors) => Outcome::Draw,
        }
    }
}

impl TryFrom<char> for Move {
    type Error = Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        let outcome = match value {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
            _ => return Err(Error::msg(format!("{} is not a valid move", value))),
        };

        Ok(outcome)
    }
}

fn parse(input: &str) -> IResult<&str, Vec<(char, char)>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, (char, char)> {
    separated_pair(one_of("ABC"), complete::char(' '), one_of("XYZ"))(input)
}

fn read_input() -> Result<Vec<(char, char)>> {
    let mut buf = String::new();
    fs::File::open("src/input.txt")?.read_to_string(&mut buf)?;

    let (_, input) = parse(&buf).expect("Parse failure");

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let input = read_input()?;

        let count = part_one(&input)?;

        assert_eq!(10718, count);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input()?;

        let count = part_two(&input)?;

        assert_eq!(14652, count);

        Ok(())
    }
}
