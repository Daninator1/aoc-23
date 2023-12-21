use std::str::FromStr;
use itertools::Itertools;
advent_of_code::solution!(15);

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}

#[derive(Debug)]
enum Operation {
    Remove(usize, String),
    Add(usize, String, usize),
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('=') {
            let (label, focal_length) = s.split('=').tuples().next().unwrap();
            let box_idx = calc_hash(label);
            Ok(Operation::Add(box_idx, label.into(), focal_length.replace('\n', "").parse::<usize>().unwrap()))
        } else if s.contains('-') {
            let label = s.split('-').next().unwrap();
            let box_idx = calc_hash(label);
            Ok(Operation::Remove(box_idx, label.into()))
        } else { panic!("invalid step") }
    }
}

fn calc_hash(input: &str) -> usize {
    input
        .chars()
        .filter(|c| c != &'\n')
        .map(|c| c as usize)
        .fold(0, |acc, ascii| ((acc + ascii) * 17) % 256)
}

pub fn part_one(input: &str) -> Option<usize> {
    let result = input.split(',').map(calc_hash).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut boxes: Vec<Vec<Lens>> = vec![vec!(); 256];
    let operations: Vec<Operation> = input.split(',').map(|step| Operation::from_str(step).unwrap()).collect();

    for operation in operations {
        match operation {
            Operation::Remove(box_idx, label) => {
                if let Some((lens_idx_to_remove, _)) = boxes[box_idx].iter().find_position(|lens| lens.label == label) {
                    boxes[box_idx].remove(lens_idx_to_remove);
                }
            }
            Operation::Add(box_idx, label, focal_length) => {
                if let Some((lens_idx_to_replace, _)) = boxes[box_idx].iter().find_position(|lens| lens.label == label) {
                    boxes[box_idx][lens_idx_to_replace] = Lens { label, focal_length }
                } else {
                    boxes[box_idx].push(Lens { label, focal_length });
                }
            }
        }
    }

    let result = boxes
        .iter().enumerate()
        .map(|(box_idx, lenses)| lenses
            .iter().enumerate()
            .map(|(lens_idx, lens)| (box_idx + 1) * (lens_idx + 1) * lens.focal_length)
            .sum::<usize>())
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
