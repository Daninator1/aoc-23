use std::str::FromStr;
use itertools::Itertools;
advent_of_code::solution!(13);

#[derive(Debug)]
struct Pattern {
    grid: Vec<Vec<char>>,
}

impl Pattern {
    fn get_reflection_value(&self) -> usize {
        Pattern::get_reflection_value_inner(&self.grid).map_or_else(|| {
            let rotated_grid: Vec<Vec<char>> = (0..self.grid[0].len()).map(|i| {
                self.grid.iter().rev().map(|line| line[i]).collect()
            }).collect();

            Pattern::get_reflection_value_inner(&rotated_grid).unwrap()
        }, |v| v * 100)
    }


    fn get_reflection_value_inner(grid: &Vec<Vec<char>>) -> Option<usize> {
        let duplicate_line_pairs: Vec<(usize, &[Vec<char>])> = grid.windows(2).enumerate().filter(|(_i, lines)| lines.iter().all_equal()).collect();

        for duplicate_line_pair in duplicate_line_pairs {
            let mut upper_idx = duplicate_line_pair.0;
            let mut lower_idx = duplicate_line_pair.0 + 1;

            let mut is_mirrored = true;

            dbg!(&upper_idx);
            dbg!(&lower_idx);

            while is_mirrored {
                if upper_idx == 0 { break; }
                if lower_idx == grid.len() - 1 { break; }

                upper_idx -= 1;
                lower_idx += 1;

                if let Some(upper) = grid.get(upper_idx) {
                    if let Some(lower) = grid.get(lower_idx) {
                        is_mirrored = upper == lower;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            dbg!(&is_mirrored);

            if is_mirrored { return Some(duplicate_line_pair.0 + 1); }
        }

        None
    }
}

impl FromStr for Pattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<char>> = s.lines().map(|line| {
            line.chars().collect()
        }).collect();

        Ok(Pattern { grid })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let blocks = input.split("\n\n");

    let patterns = blocks.map(|block| Pattern::from_str(block).unwrap());

    let result = patterns.enumerate().map(|(i, p)| {
        dbg!("{}", &i);
        p.get_reflection_value()
    }).sum();

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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
