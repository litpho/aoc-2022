use anyhow::Result;
use nom::bytes::complete::is_a;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::IResult;
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
    let start = find_single(input, b'S');
    let goal = find_single(input, b'E');

    let result = bfs(&start, |p| p.next_steps(input), |p| *p == goal).unwrap();

    result.len() - 1
}

fn part_two(input: &[Vec<u8>]) -> usize {
    let goal = find_single(input, b'E');
    let starting_points = find_all(input, b'a');

    starting_points
        .into_iter()
        .filter_map(|start| bfs(&start, |p| p.next_steps(input), |p| *p == goal))
        .map(|v| v.len())
        .min()
        .unwrap()
        - 1
}

fn find_single(input: &[Vec<u8>], single: u8) -> Coord {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, h)| ((x, y), h)))
        .find(|(_, h)| **h == single)
        .map(|((x, y), _)| Coord(x, y))
        .unwrap()
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
        let mut next_steps: Vec<Coord> = vec![];
        let current_height = get_height(input, self);

        // up
        if self.1 > 0 {
            let new_coord = Coord(self.0, self.1 - 1);
            let new_height = get_height(input, &new_coord);
            if Self::can_move(current_height, new_height) {
                next_steps.push(new_coord);
            }
        }

        // down
        if self.1 < input.len() - 1 {
            let new_coord = Coord(self.0, self.1 + 1);
            let new_height = get_height(input, &new_coord);
            if Self::can_move(current_height, new_height) {
                next_steps.push(new_coord);
            }
        }

        // left
        if self.0 > 0 {
            let new_coord = Coord(self.0 - 1, self.1);
            let new_height = get_height(input, &new_coord);
            if Self::can_move(current_height, new_height) {
                next_steps.push(new_coord);
            }
        }

        // right
        if self.0 < input.get(0).unwrap().len() - 1 {
            let new_coord = Coord(self.0 + 1, self.1);
            let new_height = get_height(input, &new_coord);
            if Self::can_move(current_height, new_height) {
                next_steps.push(new_coord);
            }
        }

        next_steps
    }

    pub fn can_move(current_height: &u8, new_height: &u8) -> bool {
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
    use test_case::test_case;

    const TESTDATA: &str = include_str!("test.txt");

    #[test_case(b'S', b'a')]
    #[test_case(b'a', b'b')]
    #[test_case(b'b', b'c')]
    #[test_case(b'c', b'd')]
    #[test_case(b'd', b'e')]
    #[test_case(b'e', b'f')]
    #[test_case(b'f', b'g')]
    #[test_case(b'g', b'h')]
    #[test_case(b'h', b'i')]
    #[test_case(b'i', b'j')]
    #[test_case(b'j', b'k')]
    #[test_case(b'k', b'l')]
    #[test_case(b'l', b'm')]
    #[test_case(b'm', b'n')]
    #[test_case(b'n', b'o')]
    #[test_case(b'o', b'p')]
    #[test_case(b'p', b'q')]
    #[test_case(b'q', b'r')]
    #[test_case(b'r', b's')]
    #[test_case(b's', b't')]
    #[test_case(b't', b'u')]
    #[test_case(b'u', b'v')]
    #[test_case(b'v', b'w')]
    #[test_case(b'w', b'x')]
    #[test_case(b'x', b'y')]
    #[test_case(b'y', b'z')]
    #[test_case(b'y', b'E')]
    #[test_case(b'z', b'E')]
    #[test_case(b'b', b'a')]
    #[test_case(b'c', b'b')]
    #[test_case(b'd', b'c')]
    #[test_case(b'e', b'd')]
    #[test_case(b'f', b'e')]
    #[test_case(b'g', b'f')]
    #[test_case(b'h', b'g')]
    #[test_case(b'i', b'h')]
    #[test_case(b'j', b'i')]
    #[test_case(b'k', b'j')]
    #[test_case(b'l', b'k')]
    #[test_case(b'm', b'l')]
    #[test_case(b'n', b'm')]
    #[test_case(b'o', b'n')]
    #[test_case(b'p', b'o')]
    #[test_case(b'q', b'p')]
    #[test_case(b'r', b'q')]
    #[test_case(b's', b'r')]
    #[test_case(b't', b's')]
    #[test_case(b'u', b't')]
    #[test_case(b'v', b'u')]
    #[test_case(b'w', b'v')]
    #[test_case(b'x', b'w')]
    #[test_case(b'y', b'x')]
    #[test_case(b'z', b'a')]
    #[test_case(b'z', b'b')]
    #[test_case(b'z', b'c')]
    #[test_case(b'z', b'd')]
    #[test_case(b'z', b'f')]
    #[test_case(b'z', b'g')]
    #[test_case(b'z', b'h')]
    #[test_case(b'z', b'i')]
    #[test_case(b'z', b'j')]
    #[test_case(b'z', b'k')]
    #[test_case(b'z', b'l')]
    #[test_case(b'z', b'm')]
    #[test_case(b'z', b'n')]
    #[test_case(b'z', b'o')]
    #[test_case(b'z', b'p')]
    #[test_case(b'z', b'q')]
    #[test_case(b'z', b'r')]
    #[test_case(b'z', b's')]
    #[test_case(b'z', b't')]
    #[test_case(b'z', b'u')]
    #[test_case(b'z', b'v')]
    #[test_case(b'z', b'w')]
    #[test_case(b'z', b'x')]
    #[test_case(b'z', b'y')]
    #[test_case(b'z', b'z')]
    #[ignore]
    fn test_part_one_coord(current_height: u8, new_height: u8) {
        assert!(Coord::can_move(&current_height, &new_height));
    }

    #[test_case(b'S', b'b')]
    #[test_case(b'a', b'c')]
    #[test_case(b'b', b'd')]
    #[test_case(b'c', b'e')]
    #[test_case(b'd', b'f')]
    #[test_case(b'e', b'g')]
    #[test_case(b'f', b'h')]
    #[test_case(b'g', b'i')]
    #[test_case(b'h', b'j')]
    #[test_case(b'i', b'k')]
    #[test_case(b'j', b'l')]
    #[test_case(b'k', b'm')]
    #[test_case(b'l', b'n')]
    #[test_case(b'm', b'o')]
    #[test_case(b'n', b'p')]
    #[test_case(b'o', b'q')]
    #[test_case(b'p', b'r')]
    #[test_case(b'q', b's')]
    #[test_case(b'r', b't')]
    #[test_case(b's', b'u')]
    #[test_case(b't', b'v')]
    #[test_case(b'u', b'w')]
    #[test_case(b'v', b'x')]
    #[test_case(b'w', b'y')]
    #[test_case(b'x', b'z')]
    #[ignore]
    fn test_part_one_coord_not(current_height: u8, new_height: u8) {
        assert!(!Coord::can_move(&current_height, &new_height));
    }

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

    #[test_case(b'E', b'z')]
    fn test_part_two_coord(current_height: u8, new_height: u8) {
        assert!(Coord::can_move_down(&current_height, &new_height));
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(29, part_two(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(0, part_two(&parse_input(DATA)?));

        Ok(())
    }
}
