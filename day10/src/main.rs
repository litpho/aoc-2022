use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::{map, value},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};
use std::collections::HashMap;
use std::ops::Rem;

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {}", took);
    let input = result?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {}", took);

    let (took, _) = took::took(|| part_two(&input));
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[Instruction]) -> i32 {
    let state = calculate_state(input);

    let mut keys = state.keys().into_iter().copied().collect::<Vec<usize>>();
    keys.sort();

    for i in keys {
        println!("{i}: {}", state.get(&i).unwrap());
    }

    println!("20: {}", get_closest(&state, 20));
    println!("60: {}", get_closest(&state, 60));
    println!("100: {}", get_closest(&state, 100));
    println!("140: {}", get_closest(&state, 140));
    println!("180: {}", get_closest(&state, 180));
    println!("220: {}", get_closest(&state, 220));

    20 * get_closest(&state, 20)
        + 60 * get_closest(&state, 60)
        + 100 * get_closest(&state, 100)
        + 140 * get_closest(&state, 140)
        + 180 * get_closest(&state, 180)
        + 220 * get_closest(&state, 220)
}

fn part_two(input: &[Instruction]) -> Vec<usize> {
    let state = calculate_state(input);
    let mut sprite: i32 = 1;
    let mut result: [bool; 241] = [false; 241];

    for (cycle, item) in result.iter_mut().enumerate().skip(1) {
        if let Some(new_sprite_x) = state.get(&(cycle - 1)) {
            println!("Move sprite to {}", new_sprite_x);
            sprite = *new_sprite_x;
        }
        if sprite.abs_diff(((cycle - 1) as i32).rem(40)) <= 1 {
            println!("Draw {cycle}");
            *item = true;
        }
    }

    for y in 0..6 {
        for x in 0..40 {
            if result[y * 40 + x + 1] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    result
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| if b { Some(i) } else { None })
        .collect::<Vec<usize>>()
}

fn calculate_state(input: &[Instruction]) -> HashMap<usize, i32> {
    let mut cycle: usize = 0;
    let mut x: i32 = 1;
    let mut state: HashMap<usize, i32> = HashMap::new();
    state.insert(0, x);
    for instruction in input.iter() {
        match instruction {
            Instruction::AddX(amount) => {
                cycle += 2;
                x += *amount;
            }
            Instruction::Noop => {
                cycle += 1;
            }
        }
        state.insert(cycle, x);
    }
    state
}

fn get_closest(input: &HashMap<usize, i32>, index: usize) -> i32 {
    if let Some(x) = input.get(&(index - 1)) {
        return *x;
    }

    *input.get(&(index - 2)).unwrap()
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    AddX(i32),
    Noop,
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Instruction> {
    alt((parse_addx, parse_noop))(input)
}

fn parse_addx(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("addx "), complete::i32), Instruction::AddX)(input)
}

fn parse_noop(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::Noop, tag("noop"))(input)
}

fn parse_input(input: &'static str) -> Result<Vec<Instruction>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    #[ignore]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(13140, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part_one() -> Result<()> {
        assert_eq!(15260, part_one(&parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        let expected = vec![
            1, 2, 5, 6, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 41, 42, 43,
            47, 48, 49, 53, 54, 55, 59, 60, 61, 65, 66, 67, 71, 72, 73, 77, 78, 79, 81, 82, 83, 84,
            89, 90, 91, 92, 97, 98, 99, 100, 105, 106, 107, 108, 113, 114, 115, 116, 121, 122, 123,
            124, 125, 131, 132, 133, 134, 135, 141, 142, 143, 144, 145, 151, 152, 153, 154, 155,
            161, 162, 163, 164, 165, 166, 173, 174, 175, 176, 177, 178, 185, 186, 187, 188, 189,
            190, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 215, 216, 217, 218, 219,
            220, 221, 229, 230, 231, 232, 233, 234, 235,
        ];
        assert_eq!(expected, part_two(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = vec![
            1, 2, 3, 7, 8, 11, 14, 16, 17, 18, 19, 22, 23, 26, 31, 34, 37, 38, 41, 44, 46, 49, 51,
            54, 56, 61, 64, 66, 71, 74, 76, 79, 81, 84, 86, 91, 92, 93, 94, 96, 97, 98, 101, 106,
            111, 114, 116, 121, 122, 123, 126, 128, 129, 131, 134, 136, 141, 143, 144, 146, 151,
            154, 156, 158, 159, 161, 166, 169, 171, 174, 176, 181, 184, 186, 191, 194, 196, 199,
            201, 207, 208, 209, 211, 214, 216, 222, 223, 224, 226, 227, 228, 229, 232, 233, 237,
            238, 239,
        ];
        assert_eq!(expected, part_two(&parse_input(DATA)?));

        Ok(())
    }
}
