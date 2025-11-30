use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::map,
    multi::separated_list1,
    IResult, Parser,
};
use std::{collections::HashSet, ops::RangeInclusive};

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {}", took);
    let input = result?;

    let (took, result) = took::took(|| part_one(&input, 2_000_000));
    println!("Result part one: {result}");
    println!("Time spent: {}", took);

    let (took, result) = took::took(|| part_two(&input, 4_000_000));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[SensorInfo], line: i32) -> usize {
    let mut result = input
        .iter()
        .filter_map(|info| info.range_on_line(line))
        .flatten()
        .collect::<HashSet<i32>>();
    input.iter().for_each(|info| {
        if info.beacon.1 == line {
            result.remove(&info.beacon.0);
        }
    });

    result.len()
}

fn part_two(input: &[SensorInfo], max: i32) -> i64 {
    for line in 0..=max {
        let ranges = input
            .iter()
            .filter_map(|info| info.range_on_line(line))
            .collect::<Vec<RangeInclusive<i32>>>();

        let (range, remainder) = join_range(&ranges);

        if !remainder.is_empty() {
            ranges.iter().for_each(|r| {
                println!("- {:?}", r);
            });
            let x = if range.end() < remainder.first().unwrap().start() {
                range.end() + 1
            } else {
                range.start() - 1
            } as i64;
            println!("({},{})", x, line);
            return x * 4_000_000 + line as i64;
        }
    }

    0
}

fn join_range(ranges: &[RangeInclusive<i32>]) -> (RangeInclusive<i32>, Vec<RangeInclusive<i32>>) {
    let mut indexes = (1..ranges.len()).collect::<HashSet<usize>>();
    let mut range = ranges[0].to_owned();
    loop {
        let mut indexes_to_remove: Vec<usize> = vec![];
        for index in indexes.iter() {
            let curr_range = &ranges[*index];
            if range.contains(curr_range.start()) {
                indexes_to_remove.push(*index);
                if !range.contains(curr_range.end()) {
                    let new_range = *range.start()..=*curr_range.end();
                    range = new_range;
                }
            } else if curr_range.contains(range.start()) {
                indexes_to_remove.push(*index);
                if !curr_range.contains(range.end()) {
                    let new_range = *curr_range.start()..=*range.end();
                    range = new_range;
                } else {
                    range = curr_range.to_owned();
                }
            }
        }
        if indexes_to_remove.is_empty() {
            let mut remainder = indexes
                .iter()
                .map(|i| ranges[*i].to_owned())
                .collect::<Vec<RangeInclusive<i32>>>();
            return if remainder.len() < 2 {
                (range, remainder)
            } else {
                remainder.push(range);
                join_range(&remainder)
            };
        } else {
            indexes_to_remove.iter().rev().for_each(|i| {
                indexes.remove(i);
            });
        }
    }
}

type Coord = (i32, i32);

#[derive(Clone, Copy, Debug)]
struct SensorInfo {
    sensor: Coord,
    beacon: Coord,
}

impl SensorInfo {
    pub fn range_on_line(&self, line: i32) -> Option<RangeInclusive<i32>> {
        let max_dist = self.manhattan_distance();
        let y_needed = self.sensor.1.abs_diff(line);
        if y_needed > max_dist {
            // out of reach
            return None;
        }
        let deviation = (max_dist - y_needed) as i32;
        Some((self.sensor.0 - deviation)..=(self.sensor.0 + deviation))
    }

    pub fn manhattan_distance(&self) -> u32 {
        self.sensor.1.abs_diff(self.beacon.1) + self.sensor.0.abs_diff(self.beacon.0)
    }
}

fn parse(input: &str) -> IResult<&str, Vec<SensorInfo>> {
    separated_list1(line_ending, parse_line).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, SensorInfo> {
    map(
        (
            tag("Sensor at "),
            parse_coord,
            tag(": closest beacon is at "),
            parse_coord,
        ),
        |(_, sensor, _, beacon)| SensorInfo { sensor, beacon },
    )
    .parse(input)
}

fn parse_coord(input: &str) -> IResult<&str, Coord> {
    map(
        (tag("x="), complete::i32, tag(", y="), complete::i32),
        |(_, x, _, y)| (x, y),
    )
    .parse(input)
}

fn parse_input(input: &'static str) -> Result<Vec<SensorInfo>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(26, part_one(&parse_input(TESTDATA)?, 10));

        Ok(())
    }

    #[test]
    // #[ignore]
    fn test_part_one() -> Result<()> {
        assert_eq!(4502208, part_one(&parse_input(DATA)?, 2_000_000));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(56000011, part_two(&parse_input(TESTDATA)?, 20));

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part_two() -> Result<()> {
        assert_eq!(13784551204480, part_two(&parse_input(DATA)?, 4_000_000));

        Ok(())
    }
}
