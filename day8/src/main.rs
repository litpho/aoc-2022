use anyhow::Result;
use nom::{
    bytes::complete::take_while1,
    character::{complete::line_ending, is_digit},
    combinator::map,
    multi::separated_list1,
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

fn part_one(input: &[Vec<u8>]) -> usize {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, height)| {
                if visible_from_left(x, height, row)
                    || visible_from_right(x, height, row)
                    || visible_from_top(x, y, height, input)
                    || visible_from_bottom(x, y, height, input)
                {
                    Some(())
                } else {
                    None
                }
            })
        })
        .count()
}

fn part_two(input: &[Vec<u8>]) -> u32 {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, height)| {
                scenic_to_left(x, height, row)
                    * scenic_to_right(x, height, row)
                    * scenic_to_top(x, y, height, input)
                    * scenic_to_bottom(x, y, height, input)
            })
        })
        .max()
        .unwrap()
}

fn visible_from_left(x: usize, height: &u8, row: &[u8]) -> bool {
    x == 0 || !(0..x).any(|x| row[x] >= *height)
}

fn visible_from_right(x: usize, height: &u8, row: &[u8]) -> bool {
    let max_x = row.len();
    x == max_x - 1 || !(x + 1..max_x).any(|x| row[x] >= *height)
}

fn visible_from_top(x: usize, y: usize, height: &u8, input: &[Vec<u8>]) -> bool {
    y == 0 || !(0..y).any(|y| input[y][x] >= *height)
}

fn visible_from_bottom(x: usize, y: usize, height: &u8, input: &[Vec<u8>]) -> bool {
    let max_y = input.len();
    y == max_y - 1 || !(y + 1..max_y).any(|y| input[y][x] >= *height)
}

fn scenic_to_left(x: usize, height: &u8, row: &[u8]) -> u32 {
    (0..x)
        .rev()
        .enumerate()
        .find(|(_, x)| row[*x] >= *height)
        .map(|(count, _)| (count + 1) as u32)
        .unwrap_or(x as u32)
}

fn scenic_to_right(x: usize, height: &u8, row: &[u8]) -> u32 {
    let max_x = row.len();
    (x + 1..max_x)
        .enumerate()
        .find(|(_, x)| row[*x] >= *height)
        .map(|(count, _)| (count + 1) as u32)
        .unwrap_or((max_x - x - 1) as u32)
}

fn scenic_to_top(x: usize, y: usize, height: &u8, input: &[Vec<u8>]) -> u32 {
    (0..y)
        .rev()
        .enumerate()
        .find(|(_, y)| input[*y][x] >= *height)
        .map(|(count, _)| (count + 1) as u32)
        .unwrap_or(y as u32)
}

fn scenic_to_bottom(x: usize, y: usize, height: &u8, input: &[Vec<u8>]) -> u32 {
    let max_y = input.len();
    (y + 1..max_y)
        .enumerate()
        .find(|(_, y)| input[*y][x] >= *height)
        .map(|(count, _)| (count + 1) as u32)
        .unwrap_or((max_y - y - 1) as u32)
}

fn parse(input: &[u8]) -> IResult<&[u8], Vec<Vec<u8>>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    map(take_while1(is_digit), |line: &[u8]| {
        line.iter().map(|b| b - b'0').collect::<Vec<u8>>()
    })(input)
}

fn parse_input(input: &'static str) -> Result<Vec<Vec<u8>>> {
    let (_, input) = parse(input.as_bytes())?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(21, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let input = parse_input(DATA)?;
        assert_eq!(1807, part_one(&input));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(8, part_two(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = parse_input(DATA)?;
        assert_eq!(480000, part_two(&input));

        Ok(())
    }
}
