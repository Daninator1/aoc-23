use std::str::FromStr;
use itertools::Itertools;
advent_of_code::solution!(22);

#[derive(Debug, Eq, PartialEq)]
struct Brick {
    id: usize,
    from: Position,
    to: Position,
}

impl Brick {
    fn create(s: &str, i: usize) -> Self {
        let (from_part, to_part) = s.split('~').tuples().next().unwrap();
        let from = Position::from_str(from_part).unwrap();
        let to = Position::from_str(to_part).unwrap();

        Brick {
            id: i,
            from,
            to,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
    z: usize,
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<usize> = s.split(',').map(|c| c.parse::<usize>().unwrap()).collect();

        Ok(Position {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        })
    }
}

fn intersects(this: (usize, usize), other: (usize, usize)) -> bool {
    this.0 <= other.1 && this.1 >= other.0
}

fn fall(bricks: &[Brick]) -> Vec<Brick> {
    let mut fallen_bricks: Vec<Brick> = vec!();

    for brick in bricks {
        let mut updated_z = brick.from.z;

        while updated_z > 1 {
            if let Some(other) = fallen_bricks.iter().find(|b| b.to.z == updated_z - 1) {
                if intersects((brick.from.x, brick.to.x), (other.from.x, other.to.x)) &&
                    intersects((brick.from.y, brick.to.y), (other.from.y, other.to.y)) {
                    break;
                }
            }

            updated_z -= 1;
        }

        let diff_z = brick.from.z - updated_z;

        fallen_bricks.push(Brick {
            id: brick.id,
            from: Position {
                x: brick.from.x,
                y: brick.from.y,
                z: brick.from.z - diff_z,
            },
            to: Position {
                x: brick.to.x,
                y: brick.to.y,
                z: brick.to.z - diff_z,
            },
        })
    }

    fallen_bricks
}

pub fn part_one(input: &str) -> Option<usize> {
    let bricks: Vec<Brick> = input
        .lines()
        .enumerate()
        .map(|(line_idx, line)| Brick::create(line, line_idx))
        .sorted_by(|a, b| a.from.z.cmp(&b.from.z))
        .collect();

    let fallen_bricks = fall(&bricks);

    let structure: Vec<(&Brick, Vec<&Brick>)> = fallen_bricks.iter().map(|b| {
        let supports = fallen_bricks
            .iter()
            .filter(|o| o.from.z == b.to.z + 1)
            .filter(|below| intersects((b.from.x, b.to.x), (below.from.x, below.to.x)) &&
                intersects((b.from.y, b.to.y), (below.from.y, below.to.y)));

        (b, supports.collect())
    }).collect();

    let result = structure.iter().flat_map(|(me, supports)| {
        if supports.is_empty() { return Some(me); }

        if supports.iter().all(|s| {
            structure.iter().any(|(x, y)| x.id != me.id && y.contains(s))
        }) { return Some(me); }

        None
    });

    Some(result.count())
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
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
