use std::str::FromStr;
use rayon::prelude::{*};
advent_of_code::solution!(17);

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<usize>> = s.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()).collect();
        Ok(Map { grid, width: s.lines().next().unwrap().chars().count(), height: s.lines().count() })
    }
}

fn calc(map: &Map, current: Position, finish: &Position) -> usize {
    *calc_rec(map, (current, Direction::Right), finish, 0, &[]).iter().min().unwrap_or(&0)
}

fn calc_rec(map: &Map, current: (Position, Direction), finish: &Position, straight_count: usize, history: &[Position]) -> Vec<usize> {
    if &current.0 == finish {
        return vec!(map.grid[current.0.y][current.0.x]);
    }

    let mut new_history = history.to_vec();
    new_history.push(current.0);

    get_next_positions(current, straight_count, map, history).iter().flat_map(|(position, direction, straight_count)| {
        calc_rec(map, (*position, *direction), finish, *straight_count, &new_history)
    }).collect()
}

fn get_next_positions(current: (Position, Direction), straight_count: usize, map: &Map, history: &[Position]) -> Vec<(Position, Direction, usize)> {
    let (position, direction) = current;

    match direction {
        Direction::Up => {
            let mut next = vec!();

            if position.y > 0 && straight_count < 3 {
                let new_position = Position { x: position.x, y: position.y - 1 };
                if !history.contains(&new_position) {
                    next.push((new_position, Direction::Up, straight_count + 1));
                }
            }

            if position.x < map.width - 1 {
                let new_position = Position { x: position.x + 1, y: position.y };
                if !history.contains(&new_position) {
                    next.push((new_position, Direction::Right, 0));
                }
            }

            if position.x > 0 {
                let new_position = Position { x: position.x - 1, y: position.y };
                if !history.contains(&new_position) {
                    next.push((new_position, Direction::Left, 0));
                }
            }

            next
        }
        Direction::Down => {
            let mut next = vec!();

            if position.y < map.height - 1 && straight_count < 3 {
                let new_position = Position { x: position.x, y: position.y + 1 };
                if !history.contains(&new_position) {
                    next.push((new_position, Direction::Down, straight_count + 1));
                }
            }

            if position.x < map.width - 1 {
                let new_position = Position { x: position.x + 1, y: position.y };
                if !history.contains(&new_position) {
                    next.push((new_position, Direction::Right, 0));
                }
            }

            if position.x > 0 {
                let new_position = Position { x: position.x - 1, y: position.y };
                if !history.contains(&new_position) {
                    next.push((new_position, Direction::Left, 0));
                }
            }

            next
        }
        Direction::Left => {
            let mut next = vec!();

            if position.x > 0 && straight_count < 3 {
                let new_position = Position { x: position.x - 1, y: position.y };
                if !history.contains(&new_position) {
                    next.push((new_position, Direction::Left, straight_count + 1));
                }
            }

            if position.y > 0 {
                let new_position = Position { x: position.x, y: position.y - 1 };
                if !history.contains(&new_position) {
                    next.push((new_position, Direction::Up, 0));
                }
            }

            if position.y < map.height - 1 {
                let new_position = Position { x: position.x, y: position.y + 1 };
                if !history.contains(&new_position) {
                    next.push((new_position, Direction::Down, 0));
                }
            }

            next
        }
        Direction::Right => {
            let mut next = vec!();

            if position.x < map.width - 1 && straight_count < 3 {
                let new_position = Position { x: position.x + 1, y: position.y };
                if !history.contains(&new_position) {
                    next.push((new_position, Direction::Right, straight_count + 1));
                }
            }

            if position.y > 0 {
                let new_position = Position { x: position.x, y: position.y - 1 };
                if !history.contains(&new_position) {
                    next.push((new_position, Direction::Up, 0));
                }
            }

            if position.y < map.height - 1 {
                let new_position = Position { x: position.x, y: position.y + 1 };
                if !history.contains(&new_position) {
                    next.push((new_position, Direction::Down, 0));
                }
            }

            next
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map::from_str(input).unwrap();
    let start = Position { x: 0, y: 0 };
    let finish = Position { x: map.width - 1, y: map.height - 1 };
    let result = calc(&map, start, &finish);
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
