use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashMap;

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

pub fn extract_word_as_number(word: &str) -> Option<&str> {
    let number_map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    for key in number_map.keys() {
        if word.contains(key) {
            return Some(number_map.get(key).unwrap());
        };
    }

    None
}

pub fn get_first_number(line: &str) -> String {
    line.chars()
        .fold_while("".to_string(), |mut curr, char| {
            if char.to_digit(10).is_some() {
                return Done(char.to_string());
            }

            curr.push(char);
            let extracted_number = extract_word_as_number(&curr);
            if extracted_number.is_some() {
                Done(extracted_number.unwrap().to_string())
            } else {
                Continue(curr)
            }
        })
        .into_inner()
}

// Getting the last number for words we can't just iterate backwards
// as the spelling of words will be reversed. There's definitely
// a more elegant way to handle this!
pub fn get_last_number(line: &str) -> String {
    line.chars()
        .rev()
        .fold_while("".to_string(), |mut curr, char| {
            if char.to_digit(10).is_some() {
                return Done(char.to_string());
            }

            curr.insert(0, char);
            let extracted_number = extract_word_as_number(&curr);
            if extracted_number.is_some() {
                Done(extracted_number.unwrap().to_string())
            } else {
                Continue(curr)
            }
        })
        .into_inner()
}

pub fn process_part2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let first_num = get_first_number(line);

            let last_num = get_last_number(line);

            String::from_iter([first_num, last_num])
                .parse::<u32>()
                .unwrap()
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_toy_input() {
        const PART_1_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

        const PART_1_EXPECTED: &str = "142";
        let result = process_part1(PART_1_INPUT);
        assert_eq!(result, PART_1_EXPECTED);
    }

    #[test]
    fn part_2_toy_input() {
        const PART_2_INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        const PART_2_EXPECTED: &str = "281";
        let result = process_part2(PART_2_INPUT);
        assert_eq!(result, PART_2_EXPECTED);
    }
}
