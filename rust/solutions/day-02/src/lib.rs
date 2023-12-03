use itertools::Itertools;

pub fn process_part1(input: &str) -> String {
    input
        .lines()
        .filter_map(|game| {
            let (label, sets) = game
                .split_once(": ")
                .expect("Game description should have ': '");

            let is_possible = sets.split("; ").all(|set| {
                set.split(", ").all(|draw| {
                    let (qty, colour) = draw
                        .split_once(" ")
                        .expect("Draw description should have a whitespace");

                    let qty_u32 = qty.parse::<u32>().expect("Should be a valid integer");

                    match colour {
                        "red" => qty_u32 <= 12,
                        "green" => qty_u32 <= 13,
                        "blue" => qty_u32 <= 14,
                        _ => panic!("Unknown colour found"),
                    }
                })
            });

            if !is_possible {
                None
            } else {
                let id = label
                    .replace("Game ", "")
                    .parse::<u32>()
                    .expect("Should be a valid integer");

                return Some(id);
            }
        })
        .sum::<u32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    input
        .lines()
        .map(|game| {
            let (_, sets) = game
                .split_once(": ")
                .expect("Game description should have ': '");

            let groups = sets
                .split("; ")
                .flat_map(|set| set.split(", "))
                .filter_map(|draw| {
                    let (qty, colour) = draw
                        .split_once(" ")
                        .expect("Draw description should have a whitespace");

                    Some((qty.parse::<u32>().unwrap(), colour))
                })
                .into_grouping_map_by(|draw| draw.1)
                .max_by(|_, a, b| a.0.cmp(&b.0));

            return groups.values().map(|(qty, _)| qty).product::<u32>();
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    const PART_1_EXPECTED: &str = "8";
    const PART_2_EXPECTED: &str = "2286";

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
