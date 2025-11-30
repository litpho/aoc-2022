use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use std::cmp::{max, min};
use std::collections::HashSet;

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

fn part_one(grid: &Grid) -> usize {
    let mut sand_grid: Grid = HashSet::new();
    let max_y = grid.iter().map(|(_, y)| y).max().unwrap();
    let min_x = grid.iter().map(|(x, _)| x).min().unwrap();
    let max_x = grid.iter().map(|(x, _)| x).max().unwrap();

    loop {
        let mut new_sand = (500, 0);
        while let Some(sand) = move_sand(&new_sand, grid, &mut sand_grid) {
            if sand.1 >= *max_y || sand.0 < *min_x || sand.0 > *max_x {
                return sand_grid.len();
            }
            new_sand = sand;
        }
    }
}

fn part_two(grid: &Grid) -> usize {
    let mut sand_grid: Grid = HashSet::new();
    let max_y = grid.iter().map(|(_, y)| y).max().unwrap() + 2;

    loop {
        let mut new_sand = (500, 0);
        while let Some(sand) = move_sand_with_floor(&new_sand, grid, &mut sand_grid, max_y) {
            new_sand = sand;
        }
        // new_sand didn't move
        if new_sand == (500, 0) {
            return sand_grid.len();
        }
    }
}

fn move_sand(sand: &Coord, grid: &Grid, sand_grid: &mut Grid) -> Option<Coord> {
    if let Some(coord) = move_down(sand, grid, sand_grid) {
        return Some(coord);
    }
    if let Some(coord) = move_left(sand, grid, sand_grid) {
        return Some(coord);
    }
    if let Some(coord) = move_right(sand, grid, sand_grid) {
        return Some(coord);
    }

    sand_grid.insert(sand.to_owned());

    None
}

fn move_sand_with_floor(
    sand: &Coord,
    grid: &Grid,
    sand_grid: &mut Grid,
    floor: u32,
) -> Option<Coord> {
    if sand.1 < floor - 1 {
        return move_sand(sand, grid, sand_grid);
    }

    sand_grid.insert(sand.to_owned());

    None
}

fn move_down(sand: &Coord, grid: &Grid, sand_grid: &Grid) -> Option<Coord> {
    let new_coord = (sand.0, sand.1 + 1);
    test_new_coord(new_coord, grid, sand_grid)
}

fn move_left(sand: &Coord, grid: &Grid, sand_grid: &Grid) -> Option<Coord> {
    let new_coord = (sand.0 - 1, sand.1 + 1);
    test_new_coord(new_coord, grid, sand_grid)
}

fn move_right(sand: &Coord, grid: &Grid, sand_grid: &Grid) -> Option<Coord> {
    let new_coord = (sand.0 + 1, sand.1 + 1);
    test_new_coord(new_coord, grid, sand_grid)
}

fn test_new_coord(new_coord: Coord, grid: &Grid, sand_grid: &Grid) -> Option<Coord> {
    if grid.contains(&new_coord) || sand_grid.contains(&new_coord) {
        None
    } else {
        Some(new_coord)
    }
}

type Coord = (u32, u32);
type Grid = HashSet<Coord>;

fn make_grid(coords: Vec<Vec<(Coord, Coord)>>) -> Grid {
    coords
        .into_iter()
        .flatten()
        .flat_map(|(from, to)| {
            if from.0 == to.0 {
                // vertical
                (min(from.1, to.1)..=max(from.1, to.1))
                    .map(|y| (from.0, y))
                    .collect::<Vec<Coord>>()
            } else {
                // horizontal
                (min(from.0, to.0)..=max(from.0, to.0))
                    .map(|x| (x, from.1))
                    .collect::<Vec<Coord>>()
            }
        })
        .collect::<HashSet<Coord>>()
}

fn parse(input: &str) -> IResult<&str, Grid> {
    map(separated_list1(line_ending, parse_line), make_grid).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<(Coord, Coord)>> {
    map(separated_list1(tag(" -> "), parse_point), |v| {
        v.windows(2)
            .map(|w| (w[0].to_owned(), w[1].to_owned()))
            .collect::<Vec<(Coord, Coord)>>()
    })
    .parse(input)
}

fn parse_point(input: &str) -> IResult<&str, Coord> {
    map(
        separated_pair(complete::u32, complete::char(','), complete::u32),
        |(x, y)| (x, y),
    )
    .parse(input)
}

fn parse_input(input: &'static str) -> Result<Grid> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(24, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(913, part_one(&parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(93, part_two(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(30762, part_two(&parse_input(DATA)?));

        Ok(())
    }
}
