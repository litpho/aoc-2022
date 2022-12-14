use anyhow::Result;
use nom::{
    bytes::complete::is_a, character::complete::line_ending, combinator::map,
    multi::separated_list1, IResult,
};
use pathfinding::prelude::bfs;

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
    let start = find_all(input, b'S')[0];
    let goal = find_all(input, b'E')[0];

    let result = bfs(&start, |p| p.next_steps(input), |p| *p == goal).unwrap();

    result.len() - 1
}

fn part_two(input: &[Vec<u8>]) -> usize {
    let goal = find_all(input, b'E')[0];
    let starting_points = find_all(input, b'a');

    starting_points
        .into_iter()
        .filter_map(|start| bfs(&start, |p| p.next_steps(input), |p| *p == goal))
        .map(|v| v.len())
        .min()
        .unwrap()
        - 1
}

fn find_all(input: &[Vec<u8>], single: u8) -> Vec<Coord> {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, h)| ((x, y), h)))
        .filter_map(|((x, y), h)| {
            if *h == single {
                Some(Coord(x, y))
            } else {
                None
            }
        })
        .collect::<Vec<Coord>>()
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
struct Coord(usize, usize);

impl Coord {
    fn next_steps(&self, input: &[Vec<u8>]) -> Vec<Coord> {
        let width = input.get(0).unwrap().len() - 1;
        let height = input.len() - 1;

        let mut next_steps = vec![];

        // up
        self.calculate_step(&mut next_steps, input, |s| s.1 > 0, |s| s, |s| s - 1);
        // down
        self.calculate_step(&mut next_steps, input, |s| s.1 < height, |s| s, |s| s + 1);
        // left
        self.calculate_step(&mut next_steps, input, |s| s.0 > 0, |s| s - 1, |s| s);
        // right
        self.calculate_step(&mut next_steps, input, |s| s.0 < width, |s| s + 1, |s| s);

        next_steps
    }

    fn calculate_step<F, G, H>(
        &self,
        next_steps: &mut Vec<Coord>,
        input: &[Vec<u8>],
        expr: F,
        x: G,
        y: H,
    ) where
        F: Fn(&Coord) -> bool,
        G: Fn(usize) -> usize,
        H: Fn(usize) -> usize,
    {
        if expr(self) {
            let new_coord = Coord(x(self.0), y(self.1));
            let current_height = get_height(input, self);
            let new_height = get_height(input, &new_coord);
            if Self::can_move(current_height, new_height) {
                next_steps.push(new_coord);
            }
        }
    }

    fn can_move(current_height: &u8, new_height: &u8) -> bool {
        (*new_height == b'E' && (*current_height == b'z' || *current_height == b'y'))
            || (*new_height == b'a' && *current_height == b'S')
            || (*new_height != b'E' && *new_height <= current_height + 1)
    }
}

fn get_height<'a>(input: &'a [Vec<u8>], coord: &Coord) -> &'a u8 {
    input.get(coord.1).unwrap().get(coord.0).unwrap()
}

fn parse(input: &[u8]) -> IResult<&[u8], Vec<Vec<u8>>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    map(is_a("abcdefghijklmnopqrstuvwxyzES"), |line: &[u8]| {
        line.to_vec()
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
        assert_eq!(31, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(425, part_one(&parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(29, part_two(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(418, part_two(&parse_input(DATA)?));

        Ok(())
    }
}
