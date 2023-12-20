use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use itertools::Itertools;
use rayon::prelude::{*};
advent_of_code::solution!(14);

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Item {
    Dish,
    Rock,
    None,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
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

fn mutate_list(list: &mut HashMap<usize, Item>, full_length: usize, dir: Direction) {
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

fn get_hash(cols: &HashMap<usize, HashMap<usize, Item>>) -> u64 {
    let flat_cols: Vec<&Item> = cols.iter().sorted_by_key(|x| x.0).flat_map(|x| x.1.iter().sorted_by_key(|x| x.0).map(|x| x.1).collect::<Vec<&Item>>()).collect();
    let mut hasher = DefaultHasher::new();
    flat_cols.hash(&mut hasher);
    hasher.finish()
}

fn cycle(init_cols: HashMap<usize, HashMap<usize, Item>>, width: usize, height: usize, cache: &mut HashMap<u64, HashMap<usize, HashMap<usize, Item>>>) -> HashMap<usize, HashMap<usize, Item>> {
    let init_cols_hash = get_hash(&init_cols);

    if let Some(cached_result) = cache.get(&init_cols_hash) {
        return cached_result.clone();
    }

    // cols
    let mut list = init_cols;

    // tilt cols north
    list.iter_mut().for_each(|(_, col)| {
        mutate_list(col, height, Direction::North);
    });

    // rows
    list = to_rows(&list, height);

    // tilt rows west
    list.iter_mut().for_each(|(_, row)| {
        mutate_list(row, width, Direction::West);
    });

    // cols
    list = to_cols(&list, width);

    // tilt cols south
    list.iter_mut().for_each(|(_, col)| {
        mutate_list(col, height, Direction::South);
    });

    // rows
    list = to_rows(&list, height);

    // tilt rows east
    list.iter_mut().for_each(|(_, row)| {
        mutate_list(row, width, Direction::East);
    });

    // cols
    let result = to_cols(&list, width);
    cache.insert(init_cols_hash, result.clone());
    result
}

pub fn part_two(input: &str) -> Option<usize> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let map: HashMap<(usize, usize), Item> = input.lines().enumerate().flat_map(|(row_idx, line)| {
        line.chars().map(Item::from).enumerate().filter(|i| i.1 != Item::None).map(|(col_idx, i)| {
            ((row_idx, col_idx), i)
        }).collect::<Vec<((usize, usize), Item)>>()
    }).collect();

    let init_cols: HashMap<usize, HashMap<usize, Item>> = (0..width).map(|y| (y, map.iter().filter(|x| x.0.1 == y).map(|x| (x.0.0, *x.1)).collect())).collect();

    let mut curr_cols = init_cols;
    let mut cache = HashMap::new();

    for _ in 0..100_000 {
        curr_cols = cycle(curr_cols, width, height, &mut cache);
    }

    dbg!(&curr_cols);

    let result = curr_cols.values().map(|result_col| calc_2(result_col, height)).sum();

    Some(result)
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
