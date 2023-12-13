use itertools::Itertools;

pub fn get_next_value(input: &Vec<Vec<i64>>) -> i64 {
    input.iter().map(|row| *row.last().unwrap()).sum()
}

pub fn get_history_pyramid(input: Vec<i64>) -> Vec<Vec<i64>> {
    let mut differences: Vec<Vec<i64>> = Vec::from([input.clone()]);
    loop {
        let source = differences.last().unwrap();
        let mut next_row = Vec::new();
        for i in 1..source.len() {
            next_row.push(source.get(i).unwrap() - source.get(i - 1).unwrap());
        }

        if next_row.clone().into_iter().all(|val| val == 0) {
            break;
        } else {
            differences.push(next_row)
        }
    }

    return differences;
}

pub fn process_part1(input: &str) -> String {
    let histories = input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|num| num.parse::<i64>().unwrap())
            .collect_vec()
    });

    let pyramids = histories.map(|h| get_history_pyramid(h)).collect_vec();
    pyramids.iter().map(get_next_value).sum::<i64>().to_string()
}

pub fn get_prev_value(input: &Vec<Vec<i64>>) -> i64 {
    input
        .iter()
        .map(|row| *row.first().unwrap())
        .rev()
        .reduce(|acc, next| next - acc)
        .unwrap()
}

pub fn process_part2(input: &str) -> String {
    let histories = input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|num| num.parse::<i64>().unwrap())
            .collect_vec()
    });

    let pyramids = histories.map(|h| get_history_pyramid(h)).collect_vec();
    pyramids.iter().map(get_prev_value).sum::<i64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    const PART_1_EXPECTED: &str = "114";
    const PART_2_EXPECTED: &str = "2";

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
