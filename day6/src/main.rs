use anyhow::Result;

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, input) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {}", took);

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
        .position(|chars| !(1..size).any(|i| chars[..i].contains(&chars[i])))
        .unwrap()
        + size
}

fn parse_input(input: &'static str) -> &'static str {
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(7, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[test_case(5, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(6, "nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    fn test_part_one_testdata(expected: usize, input: &str) {
        assert_eq!(expected, part_one(input));
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let input = parse_input(DATA);
        assert_eq!(part_one(input), 1134);

        Ok(())
    }

    #[test_case(19, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[test_case(23, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(23, "nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(29, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(26, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    fn test_part_two_testdata(expected: usize, input: &str) {
        assert_eq!(expected, part_two(input));
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = parse_input(DATA);
        assert_eq!(part_two(input), 2263);

        Ok(())
    }
}
