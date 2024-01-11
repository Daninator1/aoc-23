use std::str::FromStr;
use intersect2d::{intersect, Intersection};
use itertools::Itertools;
use num::abs;
advent_of_code::solution!(24);

#[derive(Debug, Copy, Clone)]
struct Hailstone {
    position: Position,
    velocity: Velocity,
}

impl FromStr for Hailstone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos_part, vel_part) = s.split('@').tuples().next().unwrap();

        let positions: Vec<_> = pos_part.split(',').map(|x| x.trim().parse::<f64>().unwrap()).collect();
        let velocities: Vec<_> = vel_part.split(',').map(|x| x.trim().parse::<f64>().unwrap()).collect();

        Ok(Hailstone {
            position: Position(positions[0], positions[1], positions[2]),
            velocity: Velocity(velocities[0], velocities[1], velocities[2]),
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Position(f64, f64, f64);

#[derive(Debug, Copy, Clone)]
struct Velocity(f64, f64, f64);

fn get_positions_range(hailstone: &Hailstone) -> (Position, Position) {
    let pos_a = hailstone.position;

    let pos_b = Position(hailstone.position.0 + (hailstone.velocity.0 * 1_000_000_000_000_000_000.),
                         hailstone.position.1 + (hailstone.velocity.1 * 1_000_000_000_000_000_000.),
                         hailstone.position.2);

    (pos_a, pos_b)
}

pub fn part_one(input: &str) -> Option<usize> {
    let min = 200000000000000.;
    let max = 400000000000000.;

    let hailstones = input.lines().map(|line| Hailstone::from_str(line).unwrap());

    let x: Vec<_> = hailstones.map(|stone| get_positions_range(&stone)).collect();

    let mut intersections = vec!();

    for (a, b) in x.iter().tuple_combinations() {
        let line_a = geo::Line::<f64>::from([(a.0.0, a.0.1), (a.1.0, a.1.1)]);
        let line_b = geo::Line::<f64>::from([(b.0.0, b.0.1), (b.1.0, b.1.1)]);

        if let Some(Intersection::Intersection(i)) = intersect(&line_a, &line_b) {
            if i.x >= min && i.x <= max && i.y >= min && i.y <= max {
                intersections.push(i);
            }
        }
    }

    Some(intersections.len())
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
