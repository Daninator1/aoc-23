use std::str::FromStr;
use intersect2d::{intersect, Intersection};
use itertools::Itertools;
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

pub fn part_two(input: &str) -> Option<usize> {
    let hailstones = input.lines().map(|line| Hailstone::from_str(line).unwrap());

    let (stone_a, stone_b, stone_c) = hailstones.take(3).collect_tuple().unwrap();

    let ext_2 = [
        exterior2(&subtract_velocities(&stone_a.velocity, &stone_b.velocity), &subtract_positions(&stone_a.position, &stone_b.position)),
        exterior2(&subtract_velocities(&stone_a.velocity, &stone_c.velocity), &subtract_positions(&stone_a.position, &stone_c.position)),
        exterior2(&subtract_velocities(&stone_b.velocity, &stone_c.velocity), &subtract_positions(&stone_b.position, &stone_c.position))
    ];

    let ext_3 = [
        -exterior3(&stone_a.position, &stone_a.velocity, &stone_b.position) - exterior3(&stone_b.position, &stone_b.velocity, &stone_a.position),
        -exterior3(&stone_a.position, &stone_a.velocity, &stone_c.position) - exterior3(&stone_c.position, &stone_c.velocity, &stone_a.position),
        -exterior3(&stone_b.position, &stone_b.velocity, &stone_c.position) - exterior3(&stone_c.position, &stone_c.velocity, &stone_b.position)
    ];

    let det_a =
        ext_2[0].0 * ext_2[1].1 * ext_2[2].2
            - ext_2[0].0 * ext_2[1].2 * ext_2[2].1
            - ext_2[0].1 * ext_2[1].0 * ext_2[2].2
            + ext_2[0].1 * ext_2[1].2 * ext_2[2].0
            + ext_2[0].2 * ext_2[1].0 * ext_2[2].1
            - ext_2[0].2 * ext_2[1].1 * ext_2[2].0;

    let det_ax = ext_3[0] * ext_2[1].1 * ext_2[2].2
        - ext_3[0] * ext_2[1].2 * ext_2[2].1
        - ext_2[0].1 * ext_3[1] * ext_2[2].2
        + ext_2[0].1 * ext_2[1].2 * ext_3[2]
        + ext_2[0].2 * ext_3[1] * ext_2[2].1
        - ext_2[0].2 * ext_2[1].1 * ext_3[2];

    let det_ay = ext_2[0].0 * ext_3[1] * ext_2[2].2
        - ext_2[0].0 * ext_2[1].2 * ext_3[2]
        - ext_3[0] * ext_2[1].0 * ext_2[2].2
        + ext_3[0] * ext_2[1].2 * ext_2[2].0
        + ext_2[0].2 * ext_2[1].0 * ext_3[2]
        - ext_2[0].2 * ext_3[1] * ext_2[2].0;

    let det_az = ext_2[0].0 * ext_2[1].1 * ext_3[2]
        - ext_2[0].0 * ext_3[1] * ext_2[2].1
        - ext_2[0].1 * ext_2[1].0 * ext_3[2]
        + ext_2[0].1 * ext_3[1] * ext_2[2].0
        + ext_3[0] * ext_2[1].0 * ext_2[2].1
        - ext_3[0] * ext_2[1].1 * ext_2[2].0;

    let x = det_ax / det_a;
    let y = det_ay / det_a;
    let z = det_az / det_a;

    Some((x.ceil() + y.ceil() + z.ceil()) as usize - 2)
}

fn subtract_velocities(a: &Velocity, b: &Velocity) -> Velocity {
    Velocity(a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn subtract_positions(a: &Position, b: &Position) -> Position {
    Position(a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn exterior2(velocity: &Velocity, position: &Position) -> (f64, f64, f64) {
    (velocity.0 * position.1 - velocity.1 * position.0,
     velocity.1 * position.2 - velocity.2 * position.1,
     velocity.2 * position.0 - velocity.0 * position.2, )
}

fn exterior3(a_pos: &Position, a_vel: &Velocity, b_pos: &Position) -> f64 {
    a_pos.0 * a_vel.1 * b_pos.2 + a_pos.1 * a_vel.2 * b_pos.0 + a_pos.2 * a_vel.0 * b_pos.1
        - a_pos.0 * a_vel.2 * b_pos.1
        - a_pos.1 * a_vel.0 * b_pos.2
        - a_pos.2 * a_vel.1 * b_pos.0
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
        assert_eq!(result, Some(47));
    }
}
