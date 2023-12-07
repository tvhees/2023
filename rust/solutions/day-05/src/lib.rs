use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{self, newline, space0, space1},
    multi::{many1, separated_list1},
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

    let locations = seeds.into_iter().map(|seed| {
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

fn seed_ranges(input: &str) -> IResult<&str, Vec<(i64, i64)>> {
    let (input, seeds_specs) = preceded(
        tag("seeds: "),
        many1(terminated(
            separated_pair(complete::i64, space1, complete::i64),
            space0,
        )),
    )(input)?;

    let seed_ranges = seeds_specs
        .into_iter()
        .map(|(start, count)| (start, start + count - 1));

    Ok((input, seed_ranges.collect_vec()))
}

fn apply_maps_to_ranges(ranges: Vec<(i64, i64)>, map_specs: Vec<RangeSpec>) -> Vec<(i64, i64)> {
    let mut seeds = ranges.clone();
    let mut result = Vec::new();

    for spec in map_specs {
        let source_range = (spec.source_start, spec.source_start + spec.length - 1);
        let dest_offset = spec.dest_start - spec.source_start;
        let mut remaining = Vec::new();

        for range in seeds {
            let left = (range.0, range.1.min(source_range.0 - 1));
            let overlap = (
                range.0.max(source_range.0) + dest_offset,
                range.1.min(source_range.1) + dest_offset,
            );
            let right = (range.0.max(source_range.1 + 1), range.1);

            for res in [left, right] {
                if res.0 <= res.1 {
                    remaining.push(res);
                }
            }

            if overlap.0 <= overlap.1 {
                result.push(overlap);
            }
        }

        seeds = remaining;
    }

    result.append(&mut seeds.clone());

    return result.clone();
}

pub fn process_part2(input: &str) -> String {
    // let seed_ranges = seed_ranges("seeds: 79 14 55 13");
    let (_, (seed_ranges, maps)) =
        separated_pair(seed_ranges, pair(newline, newline), map_blocks)(input)
            .expect("Well formed map");

    let location_ranges = maps
        .into_iter()
        .fold(seed_ranges, |current, (_name, specs)| {
            apply_maps_to_ranges(current.clone(), specs)
        });

    location_ranges
        .iter()
        .map(|range| range.0)
        .min()
        .unwrap()
        .to_string()
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
    const PART_2_EXPECTED: &str = "46";

    #[ignore]
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
