use std::collections::HashMap;
use std::str::FromStr;
use itertools::Itertools;
use rayon::prelude::{*};
advent_of_code::solution!(18);

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    amount: usize,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction_str, amount_str) = s.split(' ').tuples().next().unwrap();
        Ok(Instruction {
            direction: Direction::from(direction_str.chars().next().unwrap()),
            amount: amount_str.parse().unwrap(),
        })
    }
}

#[derive(Debug)]
struct ExtendedInstruction {
    direction: Direction,
    amount: usize,
}

impl ExtendedInstruction {
    fn to_instruction(&self) -> Instruction {
        Instruction { direction: self.direction, amount: self.amount }
    }
}

impl FromStr for ExtendedInstruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex_str = s.split(' ').last().unwrap();
        let raw_hex_str = hex_str.replace(['(', ')'], "");
        let amount = usize::from_str_radix(&raw_hex_str[1..raw_hex_str.len() - 1], 16).expect("invalid hex code");
        let direction = Direction::from(raw_hex_str.chars().last().unwrap());
        Ok(ExtendedInstruction { direction, amount })
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn inverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'U' | '3' => Direction::North,
            'D' | '1' => Direction::South,
            'L' | '2' => Direction::West,
            'R' | '0' => Direction::East,
            _ => panic!("invalid direction")
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn advance(&self, direction: &Direction) -> Position {
        match direction {
            Direction::North => Position { x: self.x, y: self.y - 1 },
            Direction::South => Position { x: self.x, y: self.y + 1 },
            Direction::West => Position { x: self.x - 1, y: self.y },
            Direction::East => Position { x: self.x + 1, y: self.y },
        }
    }
}

fn build_border(instructions: &[Instruction]) -> HashMap<Position, (Direction, Direction)> {
    let mut result = HashMap::new();

    let mut last_pos = Position { x: 0, y: 0 };
    let mut last_dir = instructions.last().unwrap().direction;

    for instruction in instructions {
        result.insert(last_pos, (last_dir.inverse(), instruction.direction));
        last_dir = instruction.direction;

        (0..instruction.amount - 1).for_each(|i| {
            let new_pos = last_pos.advance(&instruction.direction);
            result.insert(new_pos, (last_dir.inverse(), instruction.direction));
            last_pos = new_pos;
            last_dir = instruction.direction;
        });

        let new_pos = last_pos.advance(&instruction.direction);
        last_pos = new_pos;
        last_dir = instruction.direction;
    }

    result
}

fn count_inside(border: &HashMap<Position, (Direction, Direction)>) -> usize {
    let min_y = border.keys().min_by_key(|x| x.y).unwrap().y;
    let max_y = border.keys().max_by_key(|x| x.y).unwrap().y;

    let min_x = border.keys().min_by_key(|x| x.x).unwrap().x;
    let max_x = border.keys().max_by_key(|x| x.x).unwrap().x;

    (min_y..=max_y).par_bridge().map(|y| {
        let mut inside = false;
        let mut count = 0;

        (min_x..=max_x).for_each(|x| {
            if let Some(b) = border.get(&Position { x, y }) {
                match b {
                    (Direction::South, _) => { inside = !inside }
                    (_, Direction::South) => { inside = !inside }
                    _ => {}
                }
            } else if inside {
                count += 1;
            }
        });

        count
    }).sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let instructions: Vec<Instruction> = input.lines().map(|line| Instruction::from_str(line).unwrap()).collect();

    let border = build_border(&instructions);
    let inside = count_inside(&border);

    Some(border.len() + inside)
}

pub fn part_two(input: &str) -> Option<usize> {
    let instructions: Vec<Instruction> = input.lines().map(|line| ExtendedInstruction::from_str(line).unwrap().to_instruction()).collect();

    let border = build_border(&instructions);
    let inside = count_inside(&border);

    Some(border.len() + inside)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
