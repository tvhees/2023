use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
    Start,
}

pub trait Directional {
    fn left(&self) -> Option<(usize, usize)>;
    fn right(&self) -> Option<(usize, usize)>;
    fn up(&self) -> Option<(usize, usize)>;
    fn down(&self) -> Option<(usize, usize)>;
}

impl Directional for (usize, usize) {
    fn left(&self) -> Option<(usize, usize)> {
        match self.1 {
            0 => None,
            _ => Some((self.0, self.1 - 1)),
        }
    }

    fn right(&self) -> Option<(usize, usize)> {
        Some((self.0, self.1 + 1))
    }

    fn up(&self) -> Option<(usize, usize)> {
        match self.0 {
            0 => None,
            _ => Some((self.0 - 1, self.1)),
        }
    }

    fn down(&self) -> Option<(usize, usize)> {
        Some((self.0 + 1, self.1))
    }
}

pub fn path_length(
    position: (usize, usize),
    map: &HashMap<(usize, usize), char>,
    direction: Direction,
    mut tiles: HashSet<(usize, usize)>,
) -> Option<HashSet<(usize, usize)>> {
    if matches!(direction, Direction::Start) {
        return Some(tiles);
    } else if matches!(direction, Direction::None) {
        return None;
    }

    let next_position = match &direction {
        Direction::Up => position.up(),
        Direction::Down => position.down(),
        Direction::Left => position.left(),
        Direction::Right => position.right(),
        _ => None,
    };

    if next_position.is_none() {
        return None;
    }

    let next_char = map.get(&next_position.unwrap()).unwrap_or(&'.');
    let next_direction = match next_char {
        'J' => match &direction {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Left,
            _ => Direction::None,
        },
        'L' => match &direction {
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Right,
            _ => Direction::None,
        },
        'F' => match &direction {
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Right,
            _ => Direction::None,
        },
        '7' => match &direction {
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Left,
            _ => Direction::None,
        },
        '-' => direction,
        '|' => direction,
        'S' => Direction::Start,
        _ => Direction::None,
    };

    tiles.insert(position);
    path_length(next_position.unwrap(), map, next_direction, tiles)
}

pub fn process_part1(input: &str) -> String {
    let grid = input.lines().enumerate().flat_map(|(i, line)| {
        line.chars()
            .enumerate()
            .map(move |(j, char)| ((i, j), char))
    });

    let position = grid.clone().find(|entry| entry.1 == 'S').unwrap().0;

    let map: HashMap<(usize, usize), char> = HashMap::from_iter(grid);

    let mut steps: usize = 0;

    for direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        if let Some(tiles) = path_length(position, &map, direction, HashSet::new()) {
            steps = tiles.len() / 2;
            break;
        }
    }

    steps.to_string()
}

pub fn process_part2(input: &str) -> String {
    // Collect travelled tiles in hashset
    // Then for each line in grid, check how many vertical downward (|, L, J) tiles are crossed
    // to get to it. If Odd number > 0, it's inside the path
    let grid = input.lines().enumerate().flat_map(|(i, line)| {
        line.chars()
            .enumerate()
            .map(move |(j, char)| ((i, j), char))
    });

    let position = grid.clone().find(|entry| entry.1 == 'S').unwrap().0;

    let map: HashMap<(usize, usize), char> = HashMap::from_iter(grid);

    let path_positions = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .into_iter()
    .filter_map(|direction| path_length(position, &map, direction, HashSet::new()))
    .next()
    .unwrap();

    let rows = input.lines().collect_vec().len();
    let cols = map.len() / rows;

    let mut enclosed: usize = 0;

    for i in 0..rows {
        let mut pipes_crossed = 0;
        for j in 0..cols {
            if path_positions.contains(&(i, j)) {
                match map.get(&(i, j)).unwrap() {
                    // Technically I should handle whether S is a vertical pipe or not!
                    '|' | 'L' | 'J' | 'S' => pipes_crossed += 1,
                    _ => (),
                };
            } else if pipes_crossed % 2 > 0 {
                enclosed += 1;
            }
        }
    }

    enclosed.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_toy_input() {
        const INPUT: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

        const PART_1_EXPECTED: &str = "8";

        let result = process_part1(INPUT);
        assert_eq!(result, PART_1_EXPECTED);
    }

    #[test]
    fn part_2_toy_input() {
        const INPUT: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        const PART_2_EXPECTED: &str = "10";

        let result = process_part2(INPUT);
        assert_eq!(result, PART_2_EXPECTED);
    }
}
