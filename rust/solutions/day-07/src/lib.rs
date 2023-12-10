use std::cmp::Ordering;

use itertools::Itertools;

pub fn parse_input(input: &str) -> impl Iterator<Item = (&str, &str)> {
    input.lines().map(|line| line.split_once(" ").unwrap())
}

const CHAR_ORDER: &str = "AKQJT98765432";

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum HandType {
    Five,
    Four,
    House,
    Three,
    TwoPair,
    Pair,
    High,
}

pub fn hand_type_from_cards(hand: &str) -> HandType {
    let groups = hand
        .chars()
        .into_group_map_by(|&char| char)
        .into_iter()
        .map(|group| group.1.len())
        .sorted()
        .rev()
        .collect_vec();

    match groups.as_slice() {
        [5] => HandType::Five,
        [4, 1] => HandType::Four,
        [3, 2] => HandType::House,
        [3, 1, 1] => HandType::Three,
        [2, 2, 1] => HandType::TwoPair,
        [2, 1, 1, 1] => HandType::Pair,
        _ => HandType::High,
    }
}

pub fn compare_cards(a: &str, b: &str) -> Ordering {
    let a_type = hand_type_from_cards(a);
    let b_type = hand_type_from_cards(b);

    if a_type == b_type {
        let (a_char, b_char) = a
            .chars()
            .zip(b.chars())
            .find(|(c_a, c_b)| c_a != c_b)
            .unwrap();

        CHAR_ORDER
            .find(a_char)
            .unwrap()
            .cmp(&CHAR_ORDER.find(b_char).unwrap())
    } else {
        a_type.cmp(&b_type)
    }
}

pub fn process_part1(input: &str) -> String {
    parse_input(input)
        .sorted_by(|a, b| compare_cards(a.0, b.0))
        .rev()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.1.parse::<usize>().unwrap())
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(_input: &str) -> String {
    "placeholder".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    const PART_1_EXPECTED: &str = "6440";
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
