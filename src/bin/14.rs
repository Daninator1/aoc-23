use std::collections::HashMap;
use itertools::Itertools;
use rayon::prelude::{*};
advent_of_code::solution!(14);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Item {
    Dish,
    Rock,
    None,
}

impl From<char> for Item {
    fn from(value: char) -> Self {
        match value {
            'O' => Item::Dish,
            '#' => Item::Rock,
            _ => Item::None,
        }
    }
}

fn calc(list: &mut HashMap<usize, &Item>, full_length: usize) -> usize {
    let mut result = 0;

    if list.get(&0) == Some(&&Item::Dish) {
        result = full_length;
    }

    for x in 1..full_length {
        if list.get(&x) != Some(&&Item::Dish) {
            continue;
        }

        let mut new_x = x;

        for other_x in (0..=x - 1).rev() {
            if list.get(&other_x).is_some() {
                break;
            }

            new_x = other_x;
        }

        if new_x != x {
            list.remove(&x);
            list.insert(new_x, &Item::Dish);
        }

        result += full_length - new_x
    }

    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let map: HashMap<(usize, usize), Item> = input.lines().enumerate().flat_map(|(row_idx, line)| {
        line.chars().map(Item::from).enumerate().filter(|i| i.1 != Item::None).map(|(col_idx, i)| {
            ((row_idx, col_idx), i)
        }).collect::<Vec<((usize, usize), Item)>>()
    }).collect();

    let mut cols: Vec<HashMap<usize, &Item>> = (0..width).par_bridge().map(|y| map.par_iter().filter(|x| x.0.1 == y).map(|x| (x.0.0, x.1)).collect()).collect();


    let result = cols.par_iter_mut().map(|col| {
        calc(col, height)
    }).sum();

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
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
