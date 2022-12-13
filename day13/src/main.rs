use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, separated_pair};
use nom::IResult;
use std::cmp::Ordering;

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

fn part_one(input: &[(Node, Node)]) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| left <= right)
        .map(|(i, _)| {
            println!("{} is right", i + 1);
            i + 1
        })
        .sum()
}

fn part_two(input: &[(Node, Node)]) -> usize {
    let mut nodes = input
        .iter()
        .flat_map(|(left, right)| vec![left.to_owned(), right.to_owned()])
        .collect::<Vec<Node>>();
    let first_divider = Node::Nodes(vec![Node::Nodes(vec![Node::Value(2)])]);
    let second_divider = Node::Nodes(vec![Node::Nodes(vec![Node::Value(6)])]);
    nodes.push(first_divider.clone());
    nodes.push(second_divider.clone());

    nodes.sort();

    let indexes = nodes
        .iter()
        .enumerate()
        .filter_map(|(i, node)| {
            if node == &first_divider || node == &second_divider {
                Some(i + 1)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    indexes[0] * indexes[1]
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Node {
    Value(u8),
    Nodes(Vec<Node>),
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Node::Value(x), Node::Value(y)) => x.partial_cmp(y),
            (Node::Nodes(x), Node::Nodes(y)) => x.partial_cmp(y),
            (Node::Value(_), Node::Nodes(_)) => Node::Nodes(vec![self.clone()]).partial_cmp(other),
            (Node::Nodes(_), Node::Value(_)) => self.partial_cmp(&Node::Nodes(vec![other.clone()])),
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse(input: &str) -> IResult<&str, Vec<(Node, Node)>> {
    separated_list1(pair(line_ending, line_ending), parse_pair)(input)
}

fn parse_pair(input: &str) -> IResult<&str, (Node, Node)> {
    separated_pair(parse_tuple(0), line_ending, parse_tuple(0))(input)
}

fn parse_tuple(depth: isize) -> impl Fn(&str) -> IResult<&str, Node> {
    move |input: &str| map(alt((parse_empty, parse_non_empty(depth))), Node::Nodes)(input)
}

fn parse_non_empty(depth: isize) -> impl Fn(&str) -> IResult<&str, Vec<Node>> {
    move |input: &str| {
        delimited(
            complete::char('['),
            separated_list1(complete::char(','), parse_value(depth)),
            complete::char(']'),
        )(input)
    }
}

fn parse_empty(input: &str) -> IResult<&str, Vec<Node>> {
    map(tag("[]"), |_| vec![Node::Nodes(vec![])])(input)
}

fn parse_value(depth: isize) -> impl Fn(&str) -> IResult<&str, Node> {
    move |input: &str| alt((parse_tuple(depth + 1), map(complete::u8, Node::Value)))(input)
}

fn parse_input(input: &'static str) -> Result<Vec<(Node, Node)>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(13, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(5196, part_one(&parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(140, part_two(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(22134, part_two(&parse_input(DATA)?));

        Ok(())
    }
}
