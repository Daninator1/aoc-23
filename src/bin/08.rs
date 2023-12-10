use std::collections::HashMap;
use std::str::FromStr;
use itertools::Itertools;
use num::integer::lcm;
advent_of_code::solution!(8);

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct Map {
    instructions: Vec<Instruction>,
    nodes: HashMap<String, Node>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let instructions_str = lines.next().unwrap();

        let nodes: HashMap<String, Node> = lines.filter(|line| !line.is_empty()).map(|line| {
            let (node_str, child_nodes_str) = line.split('=').map(|x| x.trim()).tuples().next().unwrap();

            let child_nodes_str_cleaned = child_nodes_str.replace(['(', ')'], "");
            let (left_child_str, right_child_str) = child_nodes_str_cleaned.split(',').map(|x| x.trim()).tuples().next().unwrap();

            (node_str.into(), Node {
                id: node_str.into(),
                left_id: left_child_str.into(),
                right_id: right_child_str.into(),
            })
        }).collect();

        let instructions = instructions_str.chars().map(|c| match c {
            'R' => Instruction::Right,
            'L' => Instruction::Left,
            _ => panic!("invalid instruction"),
        });

        Ok(Map {
            instructions: instructions.collect(),
            nodes,
        })
    }
}


#[derive(Debug)]
struct Node {
    id: String,
    left_id: String,
    right_id: String,
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Map = input.parse().unwrap();

    let mut curr_node = &map.nodes["AAA"];
    let mut instr_idx: usize = 0;
    let mut steps = 0;

    while curr_node.id != "ZZZ" {
        curr_node = match map.instructions[instr_idx] {
            Instruction::Left => &map.nodes[&curr_node.left_id],
            Instruction::Right => &map.nodes[&curr_node.right_id],
        };

        instr_idx = (instr_idx + 1) % map.instructions.len();
        steps += 1;
    }

    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map: Map = input.parse().unwrap();

    let curr_nodes: Vec<&Node> = map.nodes.iter().filter(|(k, _)| k.ends_with('A')).map(|(_, v)| v).collect();

    let steps: Vec<u32> = curr_nodes.iter().map(|starting_node| {
        let mut curr_node = *starting_node;
        let mut instr_idx: usize = 0;
        let mut steps = 0;

        while !curr_node.id.ends_with('Z') {
            curr_node = match map.instructions[instr_idx] {
                Instruction::Left => &map.nodes[&curr_node.left_id],
                Instruction::Right => &map.nodes[&curr_node.right_id],
            };

            instr_idx = (instr_idx + 1) % map.instructions.len();
            steps += 1;
        }

        steps
    }).collect();

    let result = steps.iter().fold(1, |acc, &x| lcm(acc, x as u64));

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
