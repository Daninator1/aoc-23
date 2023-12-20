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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
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

fn tilt_map(map: &mut HashMap<(usize, usize), Item>, width: usize, height: usize, dir: Direction) {
    let mut lists: HashMap<usize, HashMap<usize, Item>> = match dir {
        Direction::North | Direction::South => (0..width).map(|i| (i, map.iter().filter(|x| x.0.1 == i).map(|x| (x.0.0, *x.1)).collect())).collect(),
        Direction::West | Direction::East => (0..height).map(|i| (i, map.iter().filter(|x| x.0.0 == i).map(|x| (x.0.0, *x.1)).collect())).collect(),
    };

    let full_length = match dir {
        Direction::North | Direction::South => height,
        Direction::West | Direction::East => width,
    };

    lists.iter_mut().for_each(|(_, list)| tilt_list(list, full_length, dir));
}

fn tilt_list(list: &mut HashMap<usize, Item>, full_length: usize, dir: Direction) {
    let x_range = match dir {
        Direction::North | Direction::West => itertools::Either::Left(1..full_length),
        Direction::South | Direction::East => itertools::Either::Right((0..full_length - 2).rev()),
    };

    for x in x_range {
        if list.get(&x) != Some(&Item::Dish) {
            continue;
        }

        let mut new_x = x;

        let other_x_range = match dir {
            Direction::North | Direction::West => itertools::Either::Right((0..=x - 1).rev()),
            Direction::South | Direction::East => itertools::Either::Left(x + 1..full_length),
        };

        for other_x in other_x_range {
            if list.get(&other_x).is_some() {
                break;
            }

            new_x = other_x;
        }

        if new_x != x {
            list.remove(&x);
            list.insert(new_x, Item::Dish);
        }
    }
}

fn calc_2(list: &HashMap<usize, Item>, full_length: usize) -> usize {
    let mut result = 0;

    for x in 0..full_length {
        if list.get(&x) == Some(&Item::Dish) {
            result += full_length - x;
        }
    }

    result
}

fn to_rows(cols: &HashMap<usize, HashMap<usize, Item>>, height: usize) -> HashMap<usize, HashMap<usize, Item>> {
    let mut rows: HashMap<usize, HashMap<usize, Item>> = HashMap::new();

    for x in 0..height {
        let mut row: HashMap<usize, Item> = HashMap::new();

        for y in 0..cols.len() {
            if let Some(i) = cols[&y].get(&x) {
                row.insert(y, *i);
            }
        }

        rows.insert(x, row);
    }

    rows
}

fn to_cols(rows: &HashMap<usize, HashMap<usize, Item>>, width: usize) -> HashMap<usize, HashMap<usize, Item>> {
    let mut cols: HashMap<usize, HashMap<usize, Item>> = HashMap::new();

    for y in 0..width {
        let mut col: HashMap<usize, Item> = HashMap::new();

        for x in 0..rows.len() {
            if let Some(i) = rows[&x].get(&y) {
                col.insert(x, *i);
            }
        }

        cols.insert(y, col);
    }

    cols
}

fn cycle(map: &mut HashMap<(usize, usize), Item>, width: usize, height: usize) {
    tilt_map(map, width, height, Direction::North);
    tilt_map(map, width, height, Direction::West);
    tilt_map(map, width, height, Direction::South);
    tilt_map(map, width, height, Direction::East);
}

pub fn part_two(input: &str) -> Option<usize> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let map: HashMap<(usize, usize), Item> = input.lines().enumerate().flat_map(|(row_idx, line)| {
        line.chars().map(Item::from).enumerate().filter(|i| i.1 != Item::None).map(|(col_idx, i)| {
            ((row_idx, col_idx), i)
        }).collect::<Vec<((usize, usize), Item)>>()
    }).collect();

    // let init_cols: HashMap<usize, HashMap<usize, Item>> = (0..width).map(|y| (y, map.iter().filter(|x| x.0.1 == y).map(|x| (x.0.0, *x.1)).collect())).collect();

    let mut curr_map = map;

    for _ in 0..1 {
        cycle(&mut curr_map, width, height);
    }

    dbg!(&curr_map);

    // let result = curr_cols.values().map(|result_col| calc_2(result_col, height)).sum();

    Some(0)
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
        assert_eq!(result, Some(64));
    }
}
