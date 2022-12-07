use std::{collections::HashMap, ops::AddAssign};

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::{
        complete::{self, alpha1, line_ending},
        is_alphabetic,
    },
    combinator::map,
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair},
    IResult,
};

const DATA: &str = include_str!("input.txt");
const TOTAL_SIZE: u32 = 70_000_000;
const MINIMUM_NEEDED: u32 = 30_000_000;

fn main() -> Result<()> {
    let input = parse_input(DATA)?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {}", took);

    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &HashMap<String, u32>) -> u32 {
    input
        .iter()
        .filter_map(|(_, v)| if v <= &100_000 { Some(v) } else { None })
        .sum::<u32>()
}

fn part_two(input: &HashMap<String, u32>) -> u32 {
    let total_needed = *input.get(&"/".to_owned()).unwrap() + MINIMUM_NEEDED - TOTAL_SIZE;
    *input
        .iter()
        .filter_map(|(_, v)| if v >= &total_needed { Some(v) } else { None })
        .min()
        .unwrap()
}

#[derive(Debug)]
struct File(u32);

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls(Vec<File>),
}

#[derive(Debug)]
enum LsLine {
    Dir,
    File(u32),
}

fn parse(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list1(line_ending, parse_command)(input)
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    alt((parse_cd, parse_ls))(input)
}

fn parse_ls(input: &str) -> IResult<&str, Command> {
    preceded(pair(tag("$ ls"), line_ending), parse_ls_line)(input)
}

fn parse_ls_line(input: &str) -> IResult<&str, Command> {
    map(
        separated_list1(line_ending, alt((parse_ls_dir_line, parse_ls_file_line))),
        |ls_lines| {
            let files = ls_lines
                .iter()
                .filter_map(|ls_line| {
                    if let LsLine::File(size) = ls_line {
                        Some(File(*size))
                    } else {
                        None
                    }
                })
                .collect::<Vec<File>>();
            Command::Ls(files)
        },
    )(input)
}

fn parse_ls_dir_line(input: &str) -> IResult<&str, LsLine> {
    map(preceded(tag("dir "), alpha1), |_| LsLine::Dir)(input)
}

fn parse_ls_file_line(input: &str) -> IResult<&str, LsLine> {
    map(
        separated_pair(
            complete::u32,
            complete::char(' '),
            take_while1(|c| is_alphabetic(c as u8) || c == '.'),
        ),
        |(size, _)| LsLine::File(size),
    )(input)
}

fn parse_cd(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("$ cd "),
            take_while1(|c| is_alphabetic(c as u8) || c == '.' || c == '/'),
        ),
        |s: &str| Command::Cd(s.to_owned()),
    )(input)
}

fn parse_input(input: &'static str) -> Result<HashMap<String, u32>> {
    let (_, commands) = parse(input)?;

    let mut current_path: Vec<&str> = vec![];
    let mut dirs: HashMap<String, u32> = HashMap::new();
    for command in commands.iter() {
        match command {
            Command::Cd(dir_name) => match dir_name.as_str() {
                "/" => current_path.clear(),
                ".." => {
                    current_path.pop().unwrap();
                }
                _ => {
                    current_path.push(dir_name);
                }
            },
            Command::Ls(files) => {
                let curr_size = files.iter().map(|f| f.0).sum::<u32>();

                dirs.entry("/".to_owned())
                    .or_insert(0)
                    .add_assign(curr_size);

                (0..current_path.len()).for_each(|i| {
                    let curr_path = format!("/{}", current_path[0..=i].join("/"));
                    dirs.entry(curr_path).or_insert(0).add_assign(curr_size);
                });
            }
        }
    }

    Ok(dirs)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(95437, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let input = parse_input(DATA)?;
        assert_eq!(1182909, part_one(&input));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(24933642, part_two(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = parse_input(DATA)?;
        assert_eq!(2832508, part_two(&input));

        Ok(())
    }
}
