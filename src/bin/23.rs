use std::str::FromStr;
use pathfinding::matrix::{directions, Matrix};
advent_of_code::solution!(23);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Forest,
    Path,
    Slope((isize, isize)),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
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
}

#[derive(Debug)]
struct Map {
    grid: Matrix<Tile>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s.lines().map(|line| line.chars().map(Tile::from)).collect();
        Ok(Map { grid })
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct State {
    position: (usize, usize),
    direction: (isize, isize),
    distance: usize,
    tile: Tile,
}

fn calc(map: Map, start: State, finish_pos: (usize, usize)) -> usize {
    let mut current_states = vec!(start);
    let mut finished_states = vec!();

    while !current_states.is_empty() {
        let new_states = current_states
            .iter()
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
        .filter(|(position, direction, _)| {
            // do not allow directions that would backtrack
            let is_direction_inverse = state.direction.0 == -direction.0 && state.direction.1 == -direction.1;
            !is_direction_inverse
        })
        .filter(|(position, direction, tile)| {
            // do not allow directions that would hit a forest or inverting slope
            match tile {
                Tile::Forest => false,
                Tile::Path => true,
                Tile::Slope(slope_direction) => !(slope_direction == &(-state.direction.0, -state.direction.1)),
            }
        })
        .map(|(position, direction, tile)| {
            // return successors
            State { position, direction, distance: state.distance + 1, tile }
        })
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map::from_str(input).unwrap();
    let start = State { position: (0, 1), direction: (1, 0), distance: 0, tile: Tile::Path };
    let finish_pos = (map.grid.rows - 1, map.grid.columns - 2);
    let result = calc(map, start, finish_pos);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
