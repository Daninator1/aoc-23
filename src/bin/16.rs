use std::str::FromStr;
use itertools::Itertools;
use rayon::prelude::{*};
advent_of_code::solution!(16);

#[derive(Debug)]
enum Tile {
    Empty,
    RMirror,
    LMirror,
    VSplitter,
    HSplitter,
}

impl Tile {
    fn get_new_directions(&self, direction: &Direction) -> Vec<Direction> {
        match self {
            Tile::Empty => vec!(*direction),
            Tile::RMirror => match direction {
                Direction::Up => vec!(Direction::Right),
                Direction::Down => vec!(Direction::Left),
                Direction::Left => vec!(Direction::Down),
                Direction::Right => vec!(Direction::Up),
            }
            Tile::LMirror => match direction {
                Direction::Up => vec!(Direction::Left),
                Direction::Down => vec!(Direction::Right),
                Direction::Left => vec!(Direction::Up),
                Direction::Right => vec!(Direction::Down),
            }
            Tile::VSplitter => match direction {
                Direction::Up => vec!(Direction::Up),
                Direction::Down => vec!(Direction::Down),
                Direction::Left => vec!(Direction::Up, Direction::Down),
                Direction::Right => vec!(Direction::Up, Direction::Down),
            }
            Tile::HSplitter => match direction {
                Direction::Up => vec!(Direction::Left, Direction::Right),
                Direction::Down => vec!(Direction::Left, Direction::Right),
                Direction::Left => vec!(Direction::Left),
                Direction::Right => vec!(Direction::Right),
            }
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '/' => Tile::RMirror,
            '\\' => Tile::LMirror,
            '|' => Tile::VSplitter,
            '-' => Tile::HSplitter,
            _ => panic!("unknown tile")
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
struct Ball {
    direction: Direction,
    position: Position,
}

impl Ball {
    fn advance(&self, map: &Map, history: &[(Position, Direction)]) -> Vec<Ball> {
        map.grid[self.position.y][self.position.x]
            .get_new_directions(&self.direction)
            .iter()
            .flat_map(|new_direction| {
                match self.position.move_direction(new_direction, map) {
                    None => vec!(),
                    Some(new_position) => {
                        if history.contains(&(new_position, *new_direction)) {
                            vec!()
                        } else {
                            vec!(Ball { position: new_position, direction: *new_direction })
                        }
                    }
                }
            })
            .collect()
    }
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s.lines().map(|line| line.chars().map(Tile::from).collect()).collect();
        Ok(Map { grid, width: s.lines().next().unwrap().chars().count(), height: s.lines().count() })
    }
}

fn calc(ball: &Ball, map: &Map) -> usize {
    let mut history: Vec<(Position, Direction)> = vec!((ball.position, ball.direction));
    let mut balls: Vec<Ball> = vec!(*ball);

    loop {
        let iteration_results: Vec<Ball> = balls.par_iter().flat_map(|ball| ball.advance(map, &history)).collect();

        iteration_results.iter().for_each(|b| {
            history.push((b.position, b.direction));
        });

        if iteration_results.is_empty() {
            break;
        }

        balls = iteration_results;
    }

    history.iter().map(|h| h.0).unique().count()
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map::from_str(input).unwrap();

    let ball = Ball {
        position: Position { x: 0, y: 0 },
        direction: Direction::Right,
    };

    let result = calc(&ball, &map);

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Map::from_str(input).unwrap();

    let balls: Vec<Ball> = (0..map.width).map(|x| Ball { position: Position { x, y: 0 }, direction: Direction::Down })
        .chain((0..map.width).map(|x| Ball { position: Position { x, y: map.height - 1 }, direction: Direction::Up }))
        .chain((0..map.height).map(|y| Ball { position: Position { x: 0, y }, direction: Direction::Right }))
        .chain((0..map.height).map(|y| Ball { position: Position { x: map.width - 1, y }, direction: Direction::Left }))
        .collect();

    let result = balls.par_iter().map(|ball| calc(ball, &map)).max().unwrap();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
