use std::str::FromStr;
use itertools::Itertools;
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
    width: usize,
    height: usize,
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
    x: usize,
    y: usize,
}

impl Position {
    fn move_direction(&self, direction: &Direction, map: &Map) -> Option<Position> {
        match direction {
            Direction::Up => {
                if self.y == 0 { return None; }
                Some(Position { x: self.x, y: self.y - 1 })
            }
            Direction::Down => {
                if self.y == map.height - 1 { return None; }
                Some(Position { x: self.x, y: self.y + 1 })
            }
            Direction::Left => {
                if self.x == 0 { return None; }
                Some(Position { x: self.x - 1, y: self.y })
            }
            Direction::Right => {
                if self.x == map.width - 1 { return None; }
                Some(Position { x: self.x + 1, y: self.y })
            }
        }
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start_pos = Position { x: 0, y: 0 };
        let grid = s.lines().enumerate().map(|(row_idx, line)| line.chars().enumerate().map(|(col_idx, char)| {
            let tile = Tile::from(char);
            if tile == Tile::Start {
                start_pos = Position { x: col_idx, y: row_idx };
            }
            tile
        }).collect()).collect();
        Ok(Map { grid, width: s.lines().next().unwrap().chars().count(), height: s.lines().count(), start_pos })
    }
}

fn get_new_pos(position: &Position, map: &Map) -> Vec<Position> {
    let new_positions = Direction::iter().flat_map(|dir| position.move_direction(&dir, map));

    let valid_positions = new_positions.flat_map(|new_pos| {
        let target = map.grid[new_pos.y][new_pos.x];
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
        curr_positions = curr_positions.iter().flat_map(|curr_pos| get_new_pos(curr_pos, &map)).unique().collect();
    });

    Some(curr_positions.len())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
