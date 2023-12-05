use std::collections::{btree_map::Range, BTreeMap};

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{self, line_ending, newline, space1},
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug)]
struct RangeSpec {
    dest_start: i64,
    source_start: i64,
    length: i64,
}

fn map_range(input: &str) -> IResult<&str, RangeSpec> {
    let (input, numbers) = separated_list1(space1, complete::i64)(input)?;
    Ok((
        input,
        RangeSpec {
            dest_start: numbers[0],
            source_start: numbers[1],
            length: numbers[2],
        },
    ))
}

fn map_ranges(input: &str) -> IResult<&str, Vec<RangeSpec>> {
    let (input, ranges) = separated_list1(newline, map_range)(input)?;
    Ok((input, ranges))
}

fn map_heading(input: &str) -> IResult<&str, &str> {
    let (input, maps) = terminated(take_until(" map:"), tag(" map:"))(input)?;

    Ok((input, maps))
}

fn map_block(input: &str) -> IResult<&str, (&str, Vec<RangeSpec>)> {
    let (input, map_block) = separated_pair(map_heading, newline, map_ranges)(input)?;

    Ok((input, map_block))
}

fn map_blocks(input: &str) -> IResult<&str, Vec<(&str, Vec<RangeSpec>)>> {
    let (input, map_blocks) = separated_list1(pair(newline, newline), map_block)(input)?;

    Ok((input, map_blocks))
}

fn seeds(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, seeds) = preceded(tag("seeds: "), separated_list1(space1, complete::i64))(input)?;

    Ok((input, seeds))
}

pub fn process_part1(input: &str) -> String {
    let (_, (seeds, maps)) =
        separated_pair(seeds, pair(newline, newline), map_blocks)(input).expect("Well formed map");

    let locations = seeds.iter().cloned().map(|seed| {
        maps.iter().fold(seed, |current, (_name, ranges)| {
            for range in ranges {
                let RangeSpec {
                    dest_start,
                    source_start,
                    length,
                } = *range;

                if current >= source_start && current < (source_start + length) {
                    return dest_start + current - source_start;
                }
            }

            current
        })
    });

    locations.min().unwrap().to_string()
}

pub fn process_part2(_input: &str) -> String {
    "placeholder".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    const PART_1_EXPECTED: &str = "35";
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
