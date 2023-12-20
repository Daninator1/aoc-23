use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use itertools::Itertools;
use rayon::prelude::{*};
advent_of_code::solution!(14);

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Item {
    Dish,
    Rock,
    None,
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Item::Dish => 'O',
            Item::Rock => '#',
            Item::None => '.',
        })
    }
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

fn mutate_list(list: &mut [Item], full_length: usize, dir: Direction) {
    let x_range = match dir {
        Direction::North | Direction::West => itertools::Either::Left(1..full_length),
        Direction::South | Direction::East => itertools::Either::Right((0..full_length - 1).rev()),
    };

    for x in x_range {
        if list[x] != Item::Dish {
            continue;
        }

        let mut new_x = x;

        let other_x_range = match dir {
            Direction::North | Direction::West => itertools::Either::Right((0..=x - 1).rev()),
            Direction::South | Direction::East => itertools::Either::Left(x + 1..full_length),
        };

        for other_x in other_x_range {
            if list[other_x] != Item::None {
                break;
            }

            new_x = other_x;
        }

        if new_x != x {
            list[x] = Item::None;
            list[new_x] = Item::Dish;
        }
    }
}

fn calc_2(list: &[Item]) -> usize {
    let mut result = 0;

    list.iter().enumerate().for_each(|(i, x)| {
        if x == &Item::Dish {
            result += list.len() - i;
        }
    });

    result
}

// #[cached]
fn to_rows(cols: Vec<Vec<Item>>, height: usize) -> Vec<Vec<Item>> {
    let mut rows: Vec<Vec<Item>> = Vec::new();

    for x in 0..height {
        let row: Vec<Item> = cols.iter().map(|col| col[x]).collect();
        rows.push(row);
    }

    rows
}

// #[cached]
fn to_cols(rows: Vec<Vec<Item>>, width: usize) -> Vec<Vec<Item>> {
    let mut cols: Vec<Vec<Item>> = Vec::new();

    for y in 0..width {
        let col: Vec<Item> = rows.iter().map(|row| row[y]).collect();
        cols.push(col);
    }

    cols
}

// #[cached]
fn cycle(init_cols: Vec<Vec<Item>>, width: usize, height: usize) -> Vec<Vec<Item>> {
    // cols
    let mut list = init_cols;

    // tilt cols north
    list.iter_mut().for_each(|col| {
        mutate_list(col, height, Direction::North);
    });

    // rows
    list = to_rows(list, height);

    // tilt rows west
    list.iter_mut().for_each(|row| {
        mutate_list(row, width, Direction::West);
    });

    // cols
    list = to_cols(list, width);

    // tilt cols south
    list.iter_mut().for_each(|col| {
        mutate_list(col, height, Direction::South);
    });

    // rows
    list = to_rows(list, height);

    // tilt rows east
    list.iter_mut().for_each(|row| {
        mutate_list(row, width, Direction::East);
    });

    // cols
    to_cols(list, width)
}

pub fn part_two(input: &str) -> Option<usize> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let map: Vec<(usize, usize, Item)> = input.lines().enumerate().flat_map(|(row_idx, line)| {
        line.chars().map(Item::from).enumerate().map(|(col_idx, i)| {
            (row_idx, col_idx, i)
        }).collect::<Vec<(usize, usize, Item)>>()
    }).collect();

    let init_cols: Vec<Vec<Item>> = (0..width).map(|y| map.iter().filter(|x| x.1 == y).map(|x| x.2).collect()).collect();

    let mut curr_cols = init_cols;

    let mut cache: Vec<Vec<Vec<Item>>> = Vec::new();

    let target: usize = 1000000000;

    let mut cycle_offset: usize = 0;

    for i in 0..target {
        curr_cols = cycle(curr_cols, width, height);

        // println!("Iteration: {}", i + 1);
        // to_rows(curr_cols.clone(), height).iter().for_each(|row| {
        //     row.iter().for_each(|i| print!("{}", i));
        //     println!();
        // });
        // println!();

        if let Some((cache_idx, _)) = cache.iter().find_position(|c| c == &&curr_cols) {
            cycle_offset = (target - 1 - i) % (i - cache_idx);
            println!("iteration {} is same as iteration {} (offset {})", cache_idx, i, &cycle_offset);
            break;
        }

        cache.push(curr_cols.clone());
    }

    if cycle_offset != 0 {
        for _ in 0..cycle_offset {
            curr_cols = cycle(curr_cols, width, height);
        }
    }

    let result = curr_cols.iter().map(|result_col| calc_2(result_col)).sum();

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
