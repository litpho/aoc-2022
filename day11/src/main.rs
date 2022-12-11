use std::{collections::HashMap, ops::Rem};

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {}", took);
    let mut input = result?;

    let (took, result) = took::took(|| part_one(&mut input));
    println!("Result part one: {result}");
    println!("Time spent: {}", took);

    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {}", took);
    let mut input = result?;

    let (took, _) = took::took(|| part_two(&mut input));
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &mut [Monkey]) -> u64 {
    do_it(input, 20, |worry_level| worry_level / 3)
}

fn part_two(input: &mut [Monkey]) -> u64 {
    let product = input.iter().map(|m| m.test_div).product::<u64>();
    do_it(input, 10_000, |x: u64| -> u64 { x % product })
}

fn do_it<F>(input: &mut [Monkey], num_rounds: usize, worry_modifier: F) -> u64
where
    F: Fn(u64) -> u64,
{
    let amount_of_monkeys = input.len();
    let mut inspections: HashMap<usize, usize> = HashMap::new();
    for _ in 0..num_rounds {
        for monkey_id in 0..amount_of_monkeys {
            let monkey = input.get_mut(monkey_id).unwrap();
            let mut items_thrown: HashMap<usize, Vec<u64>> = HashMap::new();
            for item in monkey.items.iter() {
                *inspections.entry(monkey.id).or_default() += 1;
                let worry_level = monkey.operation.apply(*item as u64);
                let worry_level = worry_modifier(worry_level);
                let target_monkey = if worry_level.rem(monkey.test_div) == 0 {
                    monkey.target_true
                } else {
                    monkey.target_false
                };
                items_thrown
                    .entry(target_monkey)
                    .or_default()
                    .push(worry_level);
            }
            monkey.items.clear();
            for (id, items) in items_thrown {
                input.get_mut(id).unwrap().items.extend(items);
            }
        }
    }

    let mut times = inspections.values().copied().collect::<Vec<usize>>();
    times.sort_by(|a, b| b.cmp(a));

    times[0] as u64 * times[1] as u64
}

#[derive(Clone, Debug)]
struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: Operation,
    test_div: u64,
    target_true: usize,
    target_false: usize,
}

impl Monkey {
    pub fn new(
        id: usize,
        items: Vec<u64>,
        operation: Operation,
        test_div: u64,
        target_true: usize,
        target_false: usize,
    ) -> Self {
        Monkey {
            id,
            items,
            operation,
            test_div,
            target_true,
            target_false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Addition(u64),
    Multiplication(u64),
    Squared,
}

impl Operation {
    pub fn apply(&self, operand: u64) -> u64 {
        match self {
            Operation::Addition(x) => x + operand,
            Operation::Multiplication(x) => x * operand,
            Operation::Squared => operand * operand,
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(pair(line_ending, line_ending), parse_monkey)(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    map(
        tuple((
            parse_monkey_id_line,
            parse_monkey_items_line,
            parse_operation_line,
            parse_test_div_line,
            parse_target_true_line,
            parse_target_false,
        )),
        |(id, items, operation, test_div, target_true, target_false)| {
            Monkey::new(id, items, operation, test_div, target_true, target_false)
        },
    )(input)
}

fn parse_monkey_id_line(input: &str) -> IResult<&str, usize> {
    map(
        terminated(
            delimited(tag("Monkey "), complete::u8, complete::char(':')),
            line_ending,
        ),
        |id| id as usize,
    )(input)
}

fn parse_monkey_items_line(input: &str) -> IResult<&str, Vec<u64>> {
    terminated(
        preceded(
            tag("  Starting items: "),
            separated_list1(tag(", "), complete::u64),
        ),
        line_ending,
    )(input)
}

fn parse_operation_line(input: &str) -> IResult<&str, Operation> {
    terminated(
        preceded(
            tag("  Operation: new = old "),
            alt((
                parse_self_multiplication,
                parse_addition,
                parse_multiplication,
            )),
        ),
        line_ending,
    )(input)
}

fn parse_addition(input: &str) -> IResult<&str, Operation> {
    map(preceded(tag("+ "), complete::u64), Operation::Addition)(input)
}

fn parse_multiplication(input: &str) -> IResult<&str, Operation> {
    map(
        preceded(tag("* "), complete::u64),
        Operation::Multiplication,
    )(input)
}

fn parse_self_multiplication(input: &str) -> IResult<&str, Operation> {
    value(Operation::Squared, tag("* old"))(input)
}

fn parse_test_div_line(input: &str) -> IResult<&str, u64> {
    terminated(
        preceded(tag("  Test: divisible by "), complete::u64),
        line_ending,
    )(input)
}

fn parse_target_true_line(input: &str) -> IResult<&str, usize> {
    map(
        terminated(
            preceded(tag("    If true: throw to monkey "), complete::u8),
            line_ending,
        ),
        |id| id as usize,
    )(input)
}

fn parse_target_false(input: &str) -> IResult<&str, usize> {
    map(
        preceded(tag("    If false: throw to monkey "), complete::u8),
        |id| id as usize,
    )(input)
}

fn parse_input(input: &'static str) -> Result<Vec<Monkey>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(10605, part_one(&mut parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(90882, part_one(&mut parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(2713310158, part_two(&mut parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(30893109657, part_two(&mut parse_input(DATA)?));

        Ok(())
    }
}
