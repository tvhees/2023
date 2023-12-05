use std::iter;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, line_ending, multispace0, multispace1},
    error::ParseError,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct Card {
    left: Vec<u32>,
    right: Vec<u32>,
}

// Nom utility wrapper - eat any whitespace around a parsed object
fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

// 83 86  6 31 17  9 48 53 -> [83, 86, ... , 53]
fn values(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, values) = separated_list1(multispace1, complete::u32)(input)?;
    Ok((input, values))
}

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53 -> Card { left: [41, ...], right: [83, ...] }
fn card(input: &str) -> IResult<&str, Card> {
    let (input, _id) = preceded(tuple((tag("Card"), multispace0)), digit1)(input)?;
    let (input, (left, right)) = preceded(
        tuple((tag(":"), multispace0)),
        separated_pair(values, ws(tag("|")), values),
    )(input)?;

    Ok((input, Card { left, right }))
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, cards) = separated_list1(line_ending, card)(input)?;

    Ok((input, cards))
}

pub fn process_part1(input: &str) -> String {
    let (_, cards) = parse_cards(input).expect("well formed input");
    cards
        .iter()
        .map(|Card { left, right }| {
            let count = right.iter().filter(|num| left.contains(num)).count().into();
            match count {
                0 => 0,
                _ => 2u32.pow((count - 1) as u32),
            }
        })
        .sum::<u32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, cards) = parse_cards(input).expect("well formed input");
    cards
        .iter()
        .enumerate()
        .fold(
            iter::repeat(1 as u32).take(cards.len()).collect_vec(),
            |mut acc, (i, card)| {
                let Card { left, right } = card;
                let current_count = acc[i];
                let wins: usize = right.iter().filter(|num| left.contains(num)).count().into();

                for win_offset in 1..=wins {
                    let won_index = i + win_offset;
                    if let Some(v) = acc.get(won_index) {
                        acc[won_index] = v + current_count;
                    }
                }
                return acc;
            },
        )
        .iter()
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    const PART_1_EXPECTED: &str = "13";
    const PART_2_EXPECTED: &str = "30";

    #[test]
    fn part_1_toy_input() {
        let result = process_part1(INPUT);
        assert_eq!(result, PART_1_EXPECTED);
    }

    #[test]
    fn part_2_toy_input() {
        let result = process_part2(INPUT);
        assert_eq!(result, PART_2_EXPECTED);
    }
}
