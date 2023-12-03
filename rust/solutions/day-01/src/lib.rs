use std::collections::HashMap;

pub fn process_part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|character| character.to_digit(10));

            let first = digits.next().expect("No digit found in line");
            let last = digits.last().unwrap_or(first);

            return format!("{first}{last}")
                .parse::<u32>()
                .expect("Couldn't parse formatted number");
        })
        .sum::<u32>()
        .to_string()
}

pub fn starting_number_as_digit(slice: &str) -> Option<&str> {
    let number_map: HashMap<&str, &str> = HashMap::from([
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

    number_map
        .values()
        .find(|val| slice.starts_with(*val))
        .or(number_map.keys().find_map(|key| {
            if slice.starts_with(*key) {
                number_map.get(key)
            } else {
                None
            }
        }))
        .copied()
}

pub fn process_part2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let first = (0..line.len())
                .find_map(|i| starting_number_as_digit(&line[i..]))
                .expect("Should be at least one digit or number");

            let last = (0..line.len())
                .rev()
                .find_map(|i| starting_number_as_digit(&line[i..]))
                .expect("Should be at least one digit or number going backwards");

            return format!("{first}{last}")
                .parse::<u32>()
                .expect("Couldn't parse formatted number");
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
