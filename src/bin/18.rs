use std::str::FromStr;
use itertools::Itertools;
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
    fn advance_amount(&self, direction: Direction, amount: usize) -> Position {
        match direction {
            Direction::North => Position { x: self.x, y: self.y - amount as isize },
            Direction::South => Position { x: self.x, y: self.y + amount as isize },
            Direction::West => Position { x: self.x - amount as isize, y: self.y },
            Direction::East => Position { x: self.x + amount as isize, y: self.y },
        }
    }
}

fn build_corners(instructions: &[Instruction]) -> Vec<Position> {
    let mut curr_pos = Position { x: 0, y: 0 };

    instructions.iter().map(|instruction| {
        let new_pos = curr_pos.advance_amount(instruction.direction, instruction.amount);
        curr_pos = new_pos;
        new_pos
    }).collect()
}

fn count_inside_shoelace(corners: &[Position]) -> isize {
    let mut mults: Vec<(isize, isize)> = corners.iter().tuple_windows().map(|(curr, next)| (curr.x * next.y, curr.y * next.x)).collect();
    mults.push((corners[corners.len() - 1].x * corners[0].y, corners[corners.len() - 1].y * corners[0].x));
    let (mults_a, mults_b): (Vec<isize>, Vec<isize>) = mults.iter().cloned().unzip();
    let result = (mults_a.iter().sum::<isize>() - mults_b.iter().sum::<isize>()) / 2;
    result
}

pub fn part_one(input: &str) -> Option<isize> {
    let instructions: Vec<Instruction> = input.lines().map(|line| Instruction::from_str(line).unwrap()).collect();
    let corners = build_corners(&instructions);
    let inside_area = count_inside_shoelace(&corners);
    let border_count = instructions.iter().map(|x| x.amount).sum::<usize>();
    Some(inside_area + (border_count / 2) as isize + 1)
}

pub fn part_two(input: &str) -> Option<isize> {
    let instructions: Vec<Instruction> = input.lines().map(|line| ExtendedInstruction::from_str(line).unwrap().to_instruction()).collect();
    let corners = build_corners(&instructions);
    let inside_area = count_inside_shoelace(&corners);
    let border_count = instructions.iter().map(|x| x.amount).sum::<usize>();
    Some(inside_area + (border_count / 2) as isize + 1)
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
