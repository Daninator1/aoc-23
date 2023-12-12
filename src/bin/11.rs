use itertools::Itertools;
advent_of_code::solution!(11);

#[derive(Debug)]
struct Galaxy {
    id: usize,
    pos: (usize, usize),
}

fn calc(input: &str, multiplier: usize) -> usize {
    let mut initial_galaxies: Vec<(usize, usize)> = Vec::new();
    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = (0..input.lines().next().unwrap().len() - 1).collect();

    input.lines().enumerate().for_each(|(line_idx, line)| {
        if line.chars().all(|c| c == '.') {
            empty_rows.push(line_idx);
        }

        line.chars().enumerate().for_each(|(c_idx, c)| {
            if c == '#' {
                initial_galaxies.push((line_idx, c_idx));
                empty_cols.retain(|e| e != &c_idx);
            }
        })
    });

    let galaxies: Vec<Galaxy> = initial_galaxies.iter().enumerate().map(|(idx, gal)| {
        let row_mod = empty_rows.iter().filter(|e| e < &&gal.0).count();
        let col_mod = empty_cols.iter().filter(|e| e < &&gal.1).count();

        Galaxy { id: idx + 1, pos: (gal.0 + row_mod * (multiplier - 1), gal.1 + col_mod * (multiplier - 1)) }
    }).collect();

    let result: usize = galaxies.iter().tuple_combinations().map(|(a, b)| {
        let row_diff = if a.pos.0 >= b.pos.0 { a.pos.0 - b.pos.0 } else { b.pos.0 - a.pos.0 };
        let col_diff = if a.pos.1 >= b.pos.1 { a.pos.1 - b.pos.1 } else { b.pos.1 - a.pos.1 };
        row_diff + col_diff
    }).sum();

    result
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(calc(input, 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(calc(input, 1_000_000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8410));
    }
}
