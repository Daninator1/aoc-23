use num::integer::div_ceil;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
advent_of_code::solution!(10);

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Ground,
    Start,
    Connection(Direction, Direction),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Tile::Connection(Direction::North, Direction::South),
            '-' => Tile::Connection(Direction::East, Direction::West),
            'L' => Tile::Connection(Direction::North, Direction::East),
            'J' => Tile::Connection(Direction::North, Direction::West),
            '7' => Tile::Connection(Direction::South, Direction::West),
            'F' => Tile::Connection(Direction::South, Direction::East),
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!("invalid tile character")
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, EnumIter)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn inverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

fn find_start(map: &[Vec<Tile>]) -> Option<(usize, usize)> {
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if col == &Tile::Start {
                return Some((row_idx, col_idx));
            }
        }
    }

    None
}

fn find_first_conn(map: &[Vec<Tile>], start: (usize, usize)) -> ((usize, usize), Direction) {
    for dir in Direction::iter() {
        if let Some(((x, y), Some(new_dir))) = find_conn(map, (start, dir)) {
            return ((x, y), new_dir);
        }
    }

    panic!("no connecting pipe found")
}

fn find_first_conn_2(map: &[Vec<Tile>], start: (usize, usize)) -> ((usize, usize), (Direction, Direction)) {
    for dir in Direction::iter() {
        if let Some(((x, y), Some(new_dir))) = find_conn(map, (start, dir)) {
            return ((x, y), (dir.inverse(), new_dir));
        }
    }

    panic!("no connecting pipe found")
}

fn find_conn(map: &[Vec<Tile>], conn: ((usize, usize), Direction)) -> Option<((usize, usize), Option<Direction>)> {
    let conn_dir_to = conn.1;
    let (conn_x, conn_y) = conn.0;

    match conn_dir_to {
        Direction::North => {
            match map[conn_x - 1][conn_y] {
                Tile::Connection(Direction::South, to) => Some(((conn_x - 1, conn_y), Some(to))),
                Tile::Connection(to, Direction::South) => Some(((conn_x - 1, conn_y), Some(to))),
                Tile::Start => Some(((conn_x - 1, conn_y), None)),
                _ => None,
            }
        }
        Direction::East => {
            match map[conn_x][conn_y + 1] {
                Tile::Connection(Direction::West, to) => Some(((conn_x, conn_y + 1), Some(to))),
                Tile::Connection(to, Direction::West) => Some(((conn_x, conn_y + 1), Some(to))),
                Tile::Start => Some(((conn_x, conn_y + 1), None)),
                _ => None,
            }
        }
        Direction::South => {
            match map[conn_x + 1][conn_y] {
                Tile::Connection(Direction::North, to) => Some(((conn_x + 1, conn_y), Some(to))),
                Tile::Connection(to, Direction::North) => Some(((conn_x + 1, conn_y), Some(to))),
                Tile::Start => Some(((conn_x + 1, conn_y), None)),
                _ => None,
            }
        }
        Direction::West => {
            match map[conn_x][conn_y - 1] {
                Tile::Connection(Direction::East, to) => Some(((conn_x, conn_y - 1), Some(to))),
                Tile::Connection(to, Direction::East) => Some(((conn_x, conn_y - 1), Some(to))),
                Tile::Start => Some(((conn_x, conn_y - 1), None)),
                _ => None,
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<Tile>> = input.lines().map(|line| {
        line.chars().map(Tile::from).collect()
    }).collect();

    let start_pos = find_start(&map).expect("no start tile found");

    let mut conns = vec![find_first_conn(&map, start_pos)];

    loop {
        let next = find_conn(&map, conns[conns.len() - 1]).expect("no connecting tile found");
        match next.1 {
            None => { break; }
            Some(dir) => { conns.push((next.0, dir)); }
        }
    }

    Some(div_ceil(conns.len() as u32, 2))
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<Tile>> = input.lines().map(|line| {
        line.chars().map(Tile::from).collect()
    }).collect();

    let start_pos = find_start(&map).expect("no start tile found");

    let mut conns: Vec<((usize, usize), (Direction, Direction))> = vec![find_first_conn_2(&map, start_pos)];

    loop {
        let curr = conns[conns.len() - 1];
        let next = find_conn(&map, (curr.0, curr.1.1)).expect("no connecting tile found");
        match next.1 {
            None => { break; }
            Some(dir) => { conns.push((next.0, (curr.1.1.inverse(), dir))); }
        }
    }

    conns.push((start_pos, (conns[conns.len() - 1].1.1.inverse(), conns[0].1.0.inverse())));

    let mut count = 0;

    for (row_idx, row) in map.iter().enumerate() {
        let mut inside = false;

        for (col_idx, _) in row.iter().enumerate() {
            if let Some(conn) = conns.iter().find(|c| c.0 == (row_idx, col_idx)) {
                match conn.1 {
                    (Direction::South, _) => { inside = !inside }
                    (_, Direction::South) => { inside = !inside }
                    _ => {}
                }
            } else if inside {
                count += 1;
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
