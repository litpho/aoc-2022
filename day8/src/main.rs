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
    let input = parse_input(DATA)?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {}", took);

    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[Vec<u8>]) -> u32 {
    let mut count: u32 = 0;
    for (y, row) in input.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if visible_from_left(x, height, row)
                || visible_from_right(x, height, row)
                || visible_from_top(x, y, height, input)
                || visible_from_bottom(x, y, height, input)
            {
                count += 1;
            }
        }
    }
    count
}

fn part_two(input: &[Vec<u8>]) -> u32 {
    let mut scenic_max: u32 = 0;
    for (y, row) in input.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            let scenic = scenic_to_left(x, height, row)
                * scenic_to_right(x, height, row)
                * scenic_to_top(x, y, height, input)
                * scenic_to_bottom(x, y, height, input);
            if scenic > scenic_max {
                scenic_max = scenic
            }
        }
    }
    scenic_max
}

fn visible_from_left(x: usize, height: &u8, row: &[u8]) -> bool {
    x == 0 || !(0..x).any(|x| row.get(x).unwrap() >= height)
}

fn visible_from_right(x: usize, height: &u8, row: &[u8]) -> bool {
    let max_x = row.len();
    x == max_x - 1 || !(x + 1..max_x).any(|x| row.get(x).unwrap() >= height)
}

fn visible_from_top(x: usize, y: usize, height: &u8, input: &[Vec<u8>]) -> bool {
    y == 0 || !(0..y).any(|y| input.get(y).unwrap().get(x).unwrap() >= height)
}

fn visible_from_bottom(x: usize, y: usize, height: &u8, input: &[Vec<u8>]) -> bool {
    let max_y = input.len();
    y == max_y - 1 || !(y + 1..max_y).any(|y| input.get(y).unwrap().get(x).unwrap() >= height)
}

fn scenic_to_left(x: usize, height: &u8, row: &[u8]) -> u32 {
    for (count, x) in (0..x).rev().into_iter().enumerate() {
        if row.get(x).unwrap() >= height {
            return (count + 1) as u32;
        }
    }
    x as u32
}

fn scenic_to_right(x: usize, height: &u8, row: &[u8]) -> u32 {
    let max_x = row.len();
    for (count, x) in (x + 1..max_x).into_iter().enumerate() {
        if row.get(x).unwrap() >= height {
            return (count + 1) as u32;
        }
    }
    (max_x - x - 1) as u32
}

fn scenic_to_top(x: usize, y: usize, height: &u8, input: &[Vec<u8>]) -> u32 {
    for (count, y) in (0..y).into_iter().rev().enumerate() {
        if input.get(y).unwrap().get(x).unwrap() >= height {
            return (count + 1) as u32;
        }
    }
    y as u32
}

fn scenic_to_bottom(x: usize, y: usize, height: &u8, input: &[Vec<u8>]) -> u32 {
    let max_y = input.len();
    for (count, y) in (y + 1..max_y).into_iter().enumerate() {
        if input.get(y).unwrap().get(x).unwrap() >= height {
            return (count + 1) as u32;
        }
    }
    (max_y - y - 1) as u32
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
