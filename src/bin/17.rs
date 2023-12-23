use std::str::FromStr;
use pathfinding::matrix::{directions, Matrix};
use pathfinding::prelude::astar;
advent_of_code::solution!(17);

#[derive(Debug)]
struct Map {
    grid: Matrix<usize>,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct State {
    position: (usize, usize),
    direction: (isize, isize),
    distance: usize,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize)).collect();
        Ok(Map { grid })
    }
}

fn get_surrounding_states<const MAX_DIST: usize>(state: &State, map: &Map, start: &State) -> Vec<(State, usize)> {
    [directions::N, directions::E, directions::S, directions::W]
        .iter()
        .flat_map(|direction| {
            // get all neighboring points within the grid
            map.grid.move_in_direction(state.position, *direction)
                .map(|position| (position, *direction, *map.grid.get(position).expect("weight at position must exist")))
        })
        .filter(|(position, direction, _)| {
            // do not allow directions that would backtrack
            let is_direction_inverse = state.direction.0 == -direction.0 && state.direction.1 == -direction.1;
            !is_direction_inverse && *position != start.position
        })
        .flat_map(|(position, direction, weight)| {
            // return successors if they are within the max allowed distance
            let distance = match state.direction == direction {
                true => state.distance + 1,
                false => 1,
            };

            match distance <= MAX_DIST {
                true => {
                    let next_state = State { position, direction, distance };
                    Some((next_state, weight))
                }
                false => None,
            }
        })
        .collect::<Vec<_>>()
}

fn get_next_state(state: &State, map: &Map) -> Vec<(State, usize)> {
    match map.grid.move_in_direction(state.position, state.direction) {
        Some(position) => {
            let weight = *map.grid.get(position).expect("weight at position must exist");
            let new_state = State { position, direction: state.direction, distance: state.distance + 1 };
            vec!((new_state, weight))
        }
        None => vec!(),
    }
}

fn calc<const MIN_DIST: usize, const MAX_DIST: usize>(map: Map, start: State, finish_pos: (usize, usize)) -> usize {
    let path = astar(
        &start,
        |state| match state.distance >= MIN_DIST || (state.direction == (0, 0)) {
            true => get_surrounding_states::<MAX_DIST>(state, &map, &start),
            false => get_next_state(state, &map),
        },
        |state| finish_pos.0.abs_diff(state.position.0) + finish_pos.1.abs_diff(state.position.1),
        |state| state.position == finish_pos && state.distance >= MIN_DIST,
    ).expect("one path must be there");

    path.1
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map::from_str(input).unwrap();
    let start = State { position: (0, 0), direction: (0, 0), distance: 0 };
    let finish_pos = (map.grid.rows - 1, map.grid.columns - 1);
    let result = calc::<1, 3>(map, start, finish_pos);
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Map::from_str(input).unwrap();
    let start = State { position: (0, 0), direction: (0, 0), distance: 0 };
    let finish_pos = (map.grid.rows - 1, map.grid.columns - 1);
    let result = calc::<4, 10>(map, start, finish_pos);
    Some(result)
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
        assert_eq!(result, Some(94));
    }
}
