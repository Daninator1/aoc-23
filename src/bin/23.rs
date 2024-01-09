use std::collections::HashMap;
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
    let structure: HashMap<Position, Node> = get_structure(&map);
    let result: Vec<_> = find_longest(&map, (0, 1), &structure, 0, vec!());
    Some(*result.iter().max().unwrap() + 1)
}

fn get_structure(map: &Map) -> HashMap<Position, Node> {
    let mut paths = vec!();

    for row in 0..map.grid.rows {
        for col in 0..map.grid.columns {
            if map.grid.get((row, col)).unwrap() == &Tile::Path { paths.push((row, col)); }
        }
    }

    let junctions = paths.iter().filter(|p| get_next_states_2(**p, 0, None, map).len() > 2);

    let mut result: HashMap<Position, Node> = junctions.map(|j| {
        let next_junctions = find_next_junctions(map, *j, (0, 1), (map.grid.rows - 1, map.grid.columns - 2));
        (*j, Node { position: *j, to: next_junctions.iter().map(|j| (j.0.position, j.1)).collect() })
    }).collect();

    result.insert((0, 1), Node { position: (0, 1), to: find_next_junctions(map, (0, 1), (0, 1), (map.grid.rows - 1, map.grid.columns - 2)).iter().map(|j| (j.0.position, j.1)).collect() });
    result.insert((map.grid.rows - 1, map.grid.columns - 2), Node { position: (map.grid.rows - 1, map.grid.columns - 2), to: find_next_junctions(map, (map.grid.rows - 1, map.grid.columns - 2), (0, 1), (map.grid.rows - 1, map.grid.columns - 2)).iter().map(|j| (j.0.position, j.1)).collect() });

    result
}

fn get_next_states_2(position: Position, distance: usize, starting_direction: Option<(isize, isize)>, map: &Map) -> Vec<State> {
    [directions::N, directions::E, directions::S, directions::W]
        .iter()
        .flat_map(|direction| {
            // get all neighboring points within the grid
            map.grid.move_in_direction(position, *direction)
                .map(|position| (position, *direction, *map.grid.get(position).expect("tile at position must exist")))
        })
        .filter(|(_, direction, _)| {
            // do not allow directions that would backtrack
            match starting_direction {
                Some(dir) => !(dir.0 == -direction.0 && dir.1 == -direction.1),
                None => true,
            }
        })
        .filter(|(_, _, tile)| {
            // only allow directions that lead to a path
            matches!(tile, Tile::Path)
        })
        .map(|(position, direction, tile)| {
            // return successors
            State { position, direction, distance: distance + 1, tile }
        })
        .collect::<Vec<_>>()
}

fn find_next_junctions(map: &Map, junction: Position, start_pos: (usize, usize), finish_pos: (usize, usize)) -> Vec<(State, usize)> {
    let mut result = vec!();

    let next_states = get_next_states_2(junction, 0, None, map);

    for way in next_states {
        if way.position == finish_pos || way.position == start_pos {
            result.push((way.clone(), 1));
            continue;
        }
        let mut next_states = get_next_states(&way, map);
        if next_states.is_empty() { continue; }
        let mut current_weight = 1;
        let mut base = next_states[0].clone();

        while next_states.len() == 1 {
            base = next_states[0].clone();
            if base.position == finish_pos || base.position == start_pos {
                result.push((base.clone(), current_weight));
                break;
            }
            next_states = get_next_states(&base, map);
            current_weight += 1;
        }

        if next_states.len() > 1 {
            result.push((base.clone(), current_weight));
        }
    }

    result
}

fn find_longest(map: &Map, position: Position, structure: &HashMap<Position, Node>, current_weight: usize, visited: Vec<Position>) -> Vec<usize> {
    let node = structure.get(&position).unwrap();

    // finish
    if node.position == (map.grid.rows - 1, map.grid.columns - 2) {
        return vec!(current_weight);
    }

    let mut result = vec!();

    for to in &node.to {
        if !visited.contains(&to.0) {
            for x in find_longest(map, to.0, structure, current_weight + to.1, [visited.as_slice(), vec!(position).as_slice()].concat()) {
                result.push(x);
            }
        }
    }

    result
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
