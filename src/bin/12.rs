use std::collections::HashMap;
use std::iter;
use itertools::{Itertools};
use rayon::iter::{*};
advent_of_code::solution!(12);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum State {
    Working,
    Damaged,
    Unknown,
}

impl From<char> for State {
    fn from(value: char) -> Self {
        match value {
            '.' => State::Working,
            '#' => State::Damaged,
            '?' => State::Unknown,
            _ => panic!("invalid state")
        }
    }
}

fn calc(sequence: &[State], seq_idx: usize, checksums: &[usize], checksums_idx: usize) -> usize {
    let mut cache = HashMap::new();
    calc_cache(sequence, seq_idx, checksums, checksums_idx, &mut cache)
}

fn calc_cache(sequence: &[State], seq_idx: usize, checksums: &[usize], checksums_idx: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(cached_value) = cache.get(&(seq_idx, checksums_idx)) {
        return *cached_value;
    }

    // check if the current group can be satisfied from this position:
    let consume_group = checksums.get(checksums_idx).map_or(0, |checksum| {
        // group is long enough to fit within remaining springs
        if (seq_idx + checksum) > sequence.len() {
            return 0;
        }

        // group does not contain Working springs
        if (0..*checksum).any(|pos| sequence.get(seq_idx + pos) == Some(&State::Working)) {
            return 0;
        }

        // item after group is not a Damaged spring
        if sequence.get(seq_idx + checksum) == Some(&State::Damaged) {
            return 0;
        }

        // if none of the above checks failed, we have a group which we can consume
        calc_cache(sequence, seq_idx + checksum + 1, checksums, checksums_idx + 1, cache)
    });

    // also check if we can skip this position
    let skip = match sequence.get(seq_idx) {
        None => usize::from(checksums_idx >= checksums.len()),
        Some(State::Damaged) => 0,
        Some(_) => calc_cache(sequence, seq_idx + 1, checksums, checksums_idx, cache),
    };

    // add the consume_group and skip possibilities together, record in cache, and return
    let result = consume_group + skip;
    cache.insert((seq_idx, checksums_idx), result);
    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let lines: Vec<(Vec<State>, Vec<usize>)> = input.lines().map(|line| {
        let (first_part, second_part) = line.split(' ').tuples().next().unwrap();
        let sequence: Vec<State> = first_part.chars().map(State::from).collect();
        let checksums: Vec<usize> = second_part.split(',').map(|c| c.parse::<usize>().unwrap()).collect();
        (sequence, checksums)
    }).collect();

    let result: usize = lines.par_iter().map(|(seq, checksums)| calc(seq, 0, checksums, 0)).sum();

    Some(result)
}

fn unfold_sequence(states: &Vec<State>) -> Vec<State> {
    states.iter().chain(iter::once(&State::Unknown)).cloned().cycle().take((states.len() + 1) * 5 - 1).collect()
}

fn unfold_checksum(checksum: &Vec<usize>) -> Vec<usize> {
    checksum.iter().cloned().cycle().take(checksum.len() * 5).collect()
}

pub fn part_two(input: &str) -> Option<usize> {
    let sequences = input.lines().map(|line| {
        let (first_part, second_part) = line.split(' ').tuples().next().unwrap();
        let init_seq: Vec<State> = first_part.chars().map(State::from).collect();
        let exp_seq: Vec<State> = unfold_sequence(&init_seq);
        let init_checksum: Vec<usize> = second_part.split(',').map(|c| c.parse::<usize>().unwrap()).collect();
        let exp_checksum: Vec<usize> = unfold_checksum(&init_checksum);
        (exp_seq, exp_checksum)
    });

    let result: usize = sequences.par_bridge().map(|(seq, checksums)| calc(&seq, 0, &checksums, 0)).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
