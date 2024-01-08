use std::collections::HashMap;
use std::str::FromStr;
use pathfinding::matrix::{directions, Matrix};
use rayon::prelude::{*};
advent_of_code::solution!(23);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Forest,
    Path,
    Slope((isize, isize)),
}

impl Tile {
    fn create_part_1(value: char) -> Self {
        match value {
            '#' => Tile::Forest,
            '.' => Tile::Path,
            '^' => Tile::Slope(directions::N),
            'v' => Tile::Slope(directions::S),
            '<' => Tile::Slope(directions::W),
            '>' => Tile::Slope(directions::E),
            _ => panic!("unexpected tile"),
        }
    }

    fn create_part_2(value: char) -> Self {
        match value {
            '#' => Tile::Forest,
            '.' | '^' | 'v' | '<' | '>' => Tile::Path,
            _ => panic!("unexpected tile"),
        }
    }
}

#[derive(Debug)]
struct Map {
    grid: Matrix<Tile>,
}

impl Map {
    fn create_part_1(s: &str) -> Self {
        let grid = s.lines().map(|line| line.chars().map(Tile::create_part_1)).collect();
        Map { grid }
    }

    fn create_part_2(s: &str) -> Self {
        let grid = s.lines().map(|line| line.chars().map(Tile::create_part_2)).collect();
        Map { grid }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct State {
    position: (usize, usize),
    direction: (isize, isize),
    distance: usize,
    tile: Tile,
}

type Position = (usize, usize);

#[derive(Debug, Eq, PartialEq, Clone)]
struct Node {
    position: Position,
    to: Vec<(Position, usize)>,
}

fn build_structure(map: &Map, start: State, finish_pos: (usize, usize), cache: &mut Vec<Position>) -> Vec<Node> {
    let mut result = vec!();

    let mut current_state = start;
    let mut current_weight = 1;

    let mut next_states = vec!();

    loop {
        if cache.contains(&current_state.position) {
            return result;
        }

        if current_state.position == finish_pos {
            return result;
        }

        next_states = get_next_states(&current_state, map);

        if next_states.len() > 1 {
            // junction
            let junction_node = Node { position: current_state.position, to: next_states.iter().map(|x| (x.position, current_weight)).collect() };
            cache.push(junction_node.position);
            result.push(junction_node);
            current_weight = 1;
            break;
        } else {
            // go on
            current_weight += 1;
            current_state = next_states[0].clone();
        }
    }

    let other_nodes = next_states.into_iter().flat_map(|next_state| build_structure(map, next_state, finish_pos, cache));

    for other_node in other_nodes {
        result.push(other_node);
    }

    result
}

fn calc(map: Map, start: State, finish_pos: (usize, usize)) -> usize {
    let mut current_states = vec!(start);
    let mut finished_states = vec!();

    while !current_states.is_empty() {
        let new_states = current_states
            .par_iter()
            .flat_map(|current_state| get_next_states(current_state, &map));

        let (new_finished_states, new_unfinished_states): (Vec<_>, Vec<_>) = new_states.partition(|new_state| new_state.position == finish_pos);

        for new_finished_state in new_finished_states {
            finished_states.push(new_finished_state);
        }

        current_states = new_unfinished_states;
    }

    finished_states.iter().max_by_key(|x| x.distance).unwrap().distance
}

fn get_next_states(state: &State, map: &Map) -> Vec<State> {
    let directions = match state.tile {
        Tile::Slope(slope_direction) => vec!(slope_direction),
        _ => vec!(directions::N, directions::E, directions::S, directions::W),
    };

    directions
        .iter()
        .flat_map(|direction| {
            // get all neighboring points within the grid
            map.grid.move_in_direction(state.position, *direction)
                .map(|position| (position, *direction, *map.grid.get(position).expect("tile at position must exist")))
        })
        .filter(|(_, direction, _)| {
            // do not allow directions that would backtrack
            let is_direction_inverse = state.direction.0 == -direction.0 && state.direction.1 == -direction.1;
            !is_direction_inverse
        })
        .filter(|(_, _, tile)| {
            // do not allow directions that would hit a forest or inverting slope
            match tile {
                Tile::Forest => false,
                Tile::Path => true,
                Tile::Slope(slope_direction) => slope_direction != &(-state.direction.0, -state.direction.1),
            }
        })
        .map(|(position, direction, tile)| {
            // return successors
            State { position, direction, distance: state.distance + 1, tile }
        })
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map::create_part_1(input);
    let start = State { position: (0, 1), direction: (1, 0), distance: 0, tile: Tile::Path };
    let finish_pos = (map.grid.rows - 1, map.grid.columns - 2);
    let result = calc(map, start, finish_pos);
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Map::create_part_2(input);
    let start = State { position: (0, 1), direction: (1, 0), distance: 0, tile: Tile::Path };
    let finish_pos = (map.grid.rows - 1, map.grid.columns - 2);



    let sers = build_structure(&map, start, finish_pos, &mut vec!());

    dbg!(&sers);

    Some(1337)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
