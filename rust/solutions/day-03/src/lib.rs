use itertools::Itertools;
use std::iter;

trait SchematicCharacter {
    fn is_symbol(&self) -> bool;
}

impl SchematicCharacter for char {
    fn is_symbol(&self) -> bool {
        !(self.is_ascii_digit() || self == &'.')
    }
}

#[derive(Debug, Clone)]
struct PossibleNumber {
    start: usize,
    end: usize,
    string: String,
}

fn get_possible_numbers_for_line(line: &str) -> Vec<PossibleNumber> {
    let mut possible_numbers: Vec<PossibleNumber> = Vec::new();

    line.chars()
        .enumerate()
        .fold(String::new(), |mut acc, item| {
            let (i, character) = item;

            if character.is_ascii_digit() {
                acc.push(character);

                if i == line.len() - 1 {
                    possible_numbers.push(PossibleNumber {
                        start: i - &acc.len(),
                        end: i - 1,
                        string: acc.clone(),
                    });
                }
            } else if acc.len() > 0 {
                possible_numbers.push(PossibleNumber {
                    start: i - &acc.len(),
                    end: i - 1,
                    string: acc.clone(),
                });
                acc.clear();
            }

            return acc;
        });

    return possible_numbers;
}

pub fn process_part1(input: &str) -> String {
    let len = input.lines().next().unwrap().len();
    let empty_line = '.'.to_string().repeat(len);

    iter::once(empty_line.as_str())
        .chain(input.lines())
        .chain(iter::once(empty_line.as_str()))
        .tuple_windows::<(&str, &str, &str)>()
        .flat_map(|(prev_line, current_line, next_line)| {
            let possible_numbers = get_possible_numbers_for_line(current_line);

            possible_numbers.into_iter().filter_map(|possible_number| {
                let PossibleNumber { start, end, string } = possible_number;
                let is_part_number = prev_line
                    .chars()
                    .enumerate()
                    .chain(current_line.chars().enumerate())
                    .chain(next_line.chars().enumerate())
                    .any(|(i, character)| {
                        character.is_symbol() && i >= 1.max(start) - 1 && i <= end + 1
                    });

                if is_part_number {
                    Some(string.parse::<u32>().unwrap())
                } else {
                    None
                }
            })
        })
        .sum::<u32>()
        .to_string()
}

fn get_gear_product(numbers: Vec<PossibleNumber>, index: usize) -> Option<u32> {
    let adjacent_numbers = numbers
        .into_iter()
        .filter_map(|PossibleNumber { start, end, string }| {
            if index >= 1.max(start) - 1 && index <= end + 1 {
                Some(string.parse::<u32>().expect("A valid integer"))
            } else {
                None
            }
        })
        .collect_vec();

    if adjacent_numbers.len() == 2 {
        Some(adjacent_numbers.iter().product())
    } else {
        None
    }
}

pub fn process_part2(input: &str) -> String {
    let len = input.lines().next().unwrap().len();
    let empty_line = '.'.to_string().repeat(len);

    iter::once(empty_line.as_str())
        .chain(input.lines())
        .chain(iter::once(empty_line.as_str()))
        .tuple_windows::<(&str, &str, &str)>()
        .flat_map(|(prev_line, current_line, next_line)| {
            let possible_numbers = [
                get_possible_numbers_for_line(prev_line),
                get_possible_numbers_for_line(current_line),
                get_possible_numbers_for_line(next_line),
            ]
            .concat();

            return current_line.chars().enumerate().filter_map(move |item| {
                let (i, character) = item;

                if character == '*' {
                    return get_gear_product(possible_numbers.clone(), i);
                }

                return None;
            });
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    const PART_1_EXPECTED: &str = "4361";
    const PART_2_EXPECTED: &str = "467835";

    #[test]
    fn part_1_toy_input() {
        let result = process_part1(INPUT);
        assert_eq!(result, PART_1_EXPECTED);
    }

    #[test]
    fn part_1_reddit_example() {
        let result = process_part1(
            "........
.24..4.4
......*.",
        );
        assert_eq!(result, "8");
    }

    #[test]
    fn part_2_toy_input() {
        let result = process_part2(INPUT);
        assert_eq!(result, PART_2_EXPECTED);
    }
}
