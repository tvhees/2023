use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

pub fn race_times(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, times) = preceded(
        tuple((tag("Time:"), space1)),
        separated_list1(space1, complete::u32),
    )(input)?;

    Ok((input, times))
}

pub fn race_distances(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, distances) = preceded(
        tuple((tag("Distance:"), space1)),
        separated_list1(space1, complete::u32),
    )(input)?;

    Ok((input, distances))
}

/*
    Equation for a race: t(l - t) > d
    t: the time holding the button
    l: length of the race in ms
    d: current record

    We can solve quadratic eqn t^2 - lt + d = 0 to get the min and max allowed values of t
    t = (l +- sqrt(l^2 - 4d))/2
*/
pub fn winning_range_for_race((l, d): (f32, f32)) -> (u32, u32) {
    let root = (l.powi(2) - 4.0 * d).sqrt();
    let min = (l - root) / 2.0;
    let max = (l + root) / 2.0;
    let result = (min.floor() as u32 + 1, max.ceil() as u32 - 1);
    return result;
}

pub fn race_descriptions(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (input, (times, distances)) = separated_pair(race_times, newline, race_distances)(input)?;
    let races = times.into_iter().zip(distances).collect_vec();
    Ok((input, races))
}

pub fn process_part1(input: &str) -> String {
    let (_, races) = race_descriptions(input).expect("Well formed input");

    races
        .into_iter()
        .map(|(l, d)| winning_range_for_race((l as f32, d as f32)))
        .map(|(min, max)| max - min + 1)
        .product::<u32>()
        .to_string()
}

pub fn process_part2(_input: &str) -> String {
    "placeholder".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    const PART_1_EXPECTED: &str = "288";
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
