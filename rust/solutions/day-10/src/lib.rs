use std::collections::HashMap;

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
    count: usize,
) -> Option<usize> {
    if matches!(direction, Direction::Start) {
        return Some(count);
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

    path_length(next_position.unwrap(), map, next_direction, count + 1)
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
        if let Some(length) = path_length(position, &map, direction, 0) {
            steps = length / 2;
            break;
        }
    }

    steps.to_string()
}

pub fn process_part2(_input: &str) -> String {
    "placeholder".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    const PART_1_EXPECTED: &str = "8";
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
