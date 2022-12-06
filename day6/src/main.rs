use anyhow::Result;
use nom::Slice;

fn main() -> Result<()> {
    let input = read_input();

    let (took, result) = took::took(|| part_one(input));
    println!("Result part one: {result}");
    println!("Time spent: {}", took);

    let (took, result) = took::took(|| part_two(input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &str) -> usize {
    get_marker(input, 4)
}

fn part_two(input: &str) -> usize {
    get_marker(input, 14)
}

fn get_marker(input: &str, size: usize) -> usize {
    input
        .as_bytes()
        .windows(size)
        .position(|chars| !(1..size).any(|i| chars.slice(..i).contains(&chars[i])))
        .unwrap()
        + size
}

fn read_input() -> &'static str {
    include_str!("input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let c = 'q';
        let flag: i32 = 1 << (c as u32 - 'a' as u32);
        println!("{flag}");
        println!("{}", flag.count_ones());
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let input = read_input();

        let result = part_one(input);

        assert_eq!(1134, result);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input();

        let result = part_two(input);

        assert_eq!(2263, result);

        Ok(())
    }
}
