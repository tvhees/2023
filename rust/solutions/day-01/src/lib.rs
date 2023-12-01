pub fn process_part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            // Algorithm needs to handle lines with only one number in them
            // Therefore can't consume the first number when we find/take it
            // -- need to use peekable or generate a new Chars iterable for both
            // first and last number?
            let first_num = line
                .chars()
                .find(|char| char.to_digit(10).is_some())
                .unwrap();

            let last_num = line
                .chars()
                .rfind(|char| char.to_digit(10).is_some())
                .unwrap();

            String::from_iter([first_num, last_num])
                .parse::<u32>()
                .unwrap()
        })
        .sum::<u32>()
        .to_string()
}

pub fn process_part2(_input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    const PART_1_EXPECTED: &str = "142";
    const PART_2_EXPECTED: &str = "";

    #[test]
    fn part_1_toy_input() {
        let result = process_part1(INPUT);
        assert_eq!(result, PART_1_EXPECTED);
    }

    #[ignore] // Remove when doing part 2
    #[test]
    fn part_2_toy_input() {
        let result = process_part2(INPUT);
        assert_eq!(result, PART_2_EXPECTED);
    }
}
