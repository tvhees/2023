use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take, take_until},
    character::complete::newline,
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

// (BBB, BBB) -> tuple ("BBB", "BBB")
pub fn node_destinations(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, destinations) = delimited(
        tag("("),
        separated_pair(take(3 as usize), tag(", "), take(3 as usize)),
        tag(")"),
    )(input)?;

    Ok((input, destinations))
}

// AAA = (BBB, BBB) -> ("AAA", ("BBB", "BBB"))
pub fn node(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, node) = separated_pair(take(3 as usize), tag(" = "), node_destinations)(input)?;

    Ok((input, node))
}

pub fn direction_instructions(input: &str) -> IResult<&str, &str> {
    let (input, string) = terminated(take_until("\n"), tag("\n\n"))(input)?;

    Ok((input, string))
}

pub fn process_part1(input: &str) -> String {
    let (input, directions) = direction_instructions(input).expect("Well formed input");
    let (_, nodes) = separated_list1(newline, node)(input).expect("Well formed nodes");

    let node_map: HashMap<&str, (&str, &str)> = nodes.into_iter().collect();

    let mut current_node = node_map.get("AAA").unwrap();
    let mut steps = 0 as i64;

    for dir in directions.chars().cycle() {
        steps += 1;

        let next_node_tag = match dir {
            'L' => current_node.0,
            'R' => current_node.1,
            _ => panic!("Direction instruction should be L or R"),
        };

        if next_node_tag == "ZZZ" {
            break;
        }

        current_node = node_map.get(next_node_tag).unwrap();
    }

    steps.to_string()
}

pub fn process_part2(_input: &str) -> String {
    "placeholder".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const PART_1_EXPECTED: &str = "6";
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
