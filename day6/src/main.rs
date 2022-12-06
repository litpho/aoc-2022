use anyhow::Result;
use nom::Slice;

fn main() -> Result<()> {
    let input = read_input();

    let (took, result) = took::took(|| part_one(input));
    println!("Result part one: {}", result.expect("No answer found"));
    println!("Time spent: {}", took);

    let (took, result) = took::took(|| part_two(input));
    println!("Result part two: {}", result.expect("No answer found"));
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &str) -> Option<usize> {
    get_marker(input, 4)
}

fn part_two(input: &str) -> Option<usize> {
    get_marker(input, 14)
}

fn get_marker(input: &str, size: usize) -> Option<usize> {
    for (i, chars) in input.as_bytes().windows(size).enumerate() {
        if !(1..size).any(|i| chars.slice(..i).contains(&chars[i])) {
            return Some(i + size);
        }
    }

    None
}

fn read_input() -> &'static str {
    include_str!("input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let input = read_input();

        let result = part_one(input).expect("No answer found");

        assert_eq!(1134, result);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input();

        let result = part_two(input).expect("No answer found");

        assert_eq!(2263, result);

        Ok(())
    }
}
