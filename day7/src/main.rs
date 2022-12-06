use anyhow::Result;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::IResult;

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

fn part_one(_input: &[&str]) -> usize {
    unimplemented!()
}

fn part_two(_input: &[&str]) -> usize {
    unimplemented!()
}

fn parse(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(line_ending, tag("test"))(input)
}

fn parse_input(input: &'static str) -> Result<Vec<&str>> {
    let (_, output) = parse(input)?;

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    #[ignore]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(0, part_one(&parse(TESTDATA)?.1));

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part_one() -> Result<()> {
        let input = parse_input(DATA)?;
        assert_eq!(0, part_one(&input));

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(0, part_one(&parse(TESTDATA)?.1));

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part_two() -> Result<()> {
        let input = parse_input(DATA)?;
        assert_eq!(0, part_two(&input));

        Ok(())
    }
}
