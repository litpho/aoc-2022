use anyhow::Result;

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
        .position(|chars| !(1..size).any(|i| chars[..i].contains(&chars[i])))
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
    fn test_part_one_testdata() {
        assert_eq!(7, part_one("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, part_one("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let input = read_input();

        let result = part_one(input);

        assert_eq!(1134, result);

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() {
        assert_eq!(19, part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, part_two("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, part_two("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input();

        let result = part_two(input);

        assert_eq!(2263, result);

        Ok(())
    }
}
