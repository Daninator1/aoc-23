use std::str::FromStr;
use itertools::Itertools;
use rayon::prelude::{*};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
advent_of_code::solution!(21);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Start,
    Plot,
    Rock,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Tile::Start,
            '.' => Tile::Plot,
            '#' => Tile::Rock,
            _ => panic!("unknown tile")
        }
    }
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<Tile>>,
    width: isize,
    height: isize,
    start_pos: Position,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn move_direction(&self, direction: &Direction) -> Position {
        match direction {
            Direction::Up => {
                Position { x: self.x, y: self.y - 1 }
            }
            Direction::Down => {
                Position { x: self.x, y: self.y + 1 }
            }
            Direction::Left => {
                Position { x: self.x - 1, y: self.y }
            }
            Direction::Right => {
                Position { x: self.x + 1, y: self.y }
            }
        }
    }
}

fn my_mod(x: isize, m: isize) -> isize {
    (x % m + m) % m
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start_pos = Position { x: 0, y: 0 };
        let grid = s.lines().enumerate().map(|(row_idx, line)| line.chars().enumerate().map(|(col_idx, char)| {
            let tile = Tile::from(char);
            if tile == Tile::Start {
                start_pos = Position { x: col_idx as isize, y: row_idx as isize };
            }
            tile
        }).collect()).collect();
        Ok(Map { grid, width: s.lines().next().unwrap().chars().count() as isize, height: s.lines().count() as isize, start_pos })
    }
}

fn get_new_pos(position: &Position, map: &Map) -> Vec<Position> {
    let new_positions = Direction::iter().map(|dir| position.move_direction(&dir));

    let valid_positions = new_positions.flat_map(|new_pos| {
        let target = map.grid[my_mod(new_pos.y, map.height) as usize][my_mod(new_pos.x, map.width) as usize];
        if target != Tile::Rock {
            return Some(Position { x: new_pos.x, y: new_pos.y });
        }

        None
    });

    valid_positions.collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map::from_str(input).unwrap();

    let mut curr_positions = vec!(map.start_pos);

    (0..64).for_each(|_| {
        let temp = curr_positions.par_iter().flat_map(|curr_pos| get_new_pos(curr_pos, &map)).collect::<Vec<_>>();
        curr_positions = temp.into_iter().unique().collect();
    });

    Some(curr_positions.len())
}

fn check(numbers: Vec<isize>) -> isize {
    let mut history: Vec<Vec<isize>> = vec![numbers];

    loop {
        let new_numbers: Vec<isize> = history.last().unwrap().windows(2).map(|chunk| {
            match chunk {
                [a, b] => b - a,
                _ => 0
            }
        }).collect();

        if new_numbers.iter().all(|n| n == &0) { break; }

        history.push(new_numbers);
    }

    let result = history.iter().rfold(0, |acc, x| {
        let new_number = x.last().unwrap() + acc;
        new_number
    });

    result
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Map::from_str(input).unwrap();

    let test_steps = vec!(65, 65 + 131, 65 + 131 * 2);

    let mut test_lens: Vec<_> = test_steps.par_iter().map(|test_step| {
        let mut curr_positions = vec!(map.start_pos);

        (0..*test_step).for_each(|_| {
            let temp = curr_positions.par_iter().flat_map(|curr_pos| get_new_pos(curr_pos, &map)).collect::<Vec<_>>();
            curr_positions = temp.into_iter().unique().collect();
        });

        // 3784,
        // 33680,
        // 93366,
        // 182842,
        // 302108,

        curr_positions.len() as isize
    }).collect();

    (0..202298).for_each(|_| {
        let extrapolated = check(test_lens.clone());
        test_lens = test_lens[1..test_lens.len()].to_vec();
        test_lens.push(extrapolated);
    });

    Some(*test_lens.last().unwrap() as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1594));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
