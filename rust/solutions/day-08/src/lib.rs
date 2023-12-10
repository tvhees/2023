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
    let mut steps = 0 as u64;

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

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

// The inputs for this puzzle had special properties which allow
// the result to be calculated by finding the cyclical path length
// for each starting position, and then finding the lowest common
// multiple of those path lengths (lcm) - which will be the point
// at which every path has completed and landed on a '__Z' node
//
// I didn't work this out by inspection - looked up spoilers on reddit
pub fn process_part2(input: &str) -> String {
    let (input, directions) = direction_instructions(input).expect("Well formed input");
    let (_, nodes) = separated_list1(newline, node)(input).expect("Well formed nodes");

    let node_map: HashMap<&str, (&str, &str)> = nodes.into_iter().collect();
    let starting_node_tags = node_map.keys().filter(|tag| tag.ends_with("A"));
    let mut path_lengths: Vec<u64> = Vec::new();

    for start in starting_node_tags {
        let mut current_node = node_map.get(start).unwrap();
        let mut steps = 0 as u64;

        for dir in directions.chars().cycle() {
            steps += 1;

            let next_node_tag = match dir {
                'L' => current_node.0,
                'R' => current_node.1,
                _ => panic!("Direction instruction should be L or R"),
            };

            if next_node_tag.ends_with("Z") {
                break;
            }

            current_node = node_map.get(next_node_tag).unwrap();
        }

        path_lengths.push(steps);
    }

    let result = path_lengths
        .into_iter()
        .reduce(|acc, n| lcm(acc, n))
        .unwrap();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_toy_input() {
        const INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        const PART_1_EXPECTED: &str = "6";
        let result = process_part1(INPUT);
        assert_eq!(result, PART_1_EXPECTED);
    }

    #[test]
    fn part_2_toy_input() {
        const INPUT: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        const PART_2_EXPECTED: &str = "6";
        let result = process_part2(INPUT);
        assert_eq!(result, PART_2_EXPECTED);
    }
}
