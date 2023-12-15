use std::collections::HashMap;
use std::io::Read;
use std::iter;
use itertools::{Itertools, izip, repeat_n, unfold};
use rayon::iter::{*};
use regex::Regex;
use strum_macros::EnumIter;
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

fn get_possible_sequences(curr_states: &[State], rem_states: &[State], checksum: &[usize]) -> Option<Vec<Vec<State>>> {
    let mut calc_checksum: Vec<usize> = Vec::new();
    let mut counter: usize = 0;

    for state in curr_states.iter() {
        match state {
            State::Working => {
                if counter > 0 {
                    calc_checksum.push(counter);
                }
                counter = 0;
            }
            State::Damaged => {
                counter += 1;
            }
            State::Unknown => { panic!("unknown state encountered") }
        }
    }

    if counter > 0 { calc_checksum.push(counter); }

    // dbg!(&curr_states);
    // dbg!(&rem_states);
    // dbg!(&calc_checksum);
    // dbg!(&checksum);

    if calc_checksum.is_empty() ||
        (checksum.starts_with(calc_checksum.split_last().unwrap().1) &&
            calc_checksum.len() <= checksum.len() &&
            calc_checksum.last().unwrap() <= &checksum[calc_checksum.len() - 1] &&
            checksum[calc_checksum.len() - 1] - calc_checksum.last().unwrap() <= rem_states.len()) {
        if rem_states.is_empty() {
            if checksum == calc_checksum {
                return Some(vec!(curr_states.to_vec()));
            } else {
                return None;
            }
        }

        let next_state = rem_states[0];
        return match next_state {
            State::Unknown => {
                let mut result: Vec<Vec<State>> = Vec::new();

                let working = get_possible_sequences(curr_states.iter().chain(iter::once(&State::Working)).cloned().collect::<Vec<State>>().as_slice(), &rem_states[1..], checksum);
                if let Some(w) = working {
                    w.into_iter().for_each(|seq| result.push(seq));
                }

                let damaged = get_possible_sequences(curr_states.iter().chain(iter::once(&State::Damaged)).cloned().collect::<Vec<State>>().as_slice(), &rem_states[1..], checksum);
                if let Some(d) = damaged {
                    d.into_iter().for_each(|seq| result.push(seq));
                }

                if result.len() > 0 {
                    Some(result)
                } else {
                    None
                }
            }
            known => {
                get_possible_sequences(curr_states.iter().chain(iter::once(&known)).cloned().collect::<Vec<State>>().as_slice(), &rem_states[1..], checksum)
            }
        };
    } else {
        None
    }
}

fn calc(sequence: &[State], checksums: &[usize]) -> usize {
    let mut checksums_idx: usize = 0;
    let mut next_idx: usize = 0;
    let mut count: usize = 0;

    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

    for i in 0..sequence.len() {
        if i < next_idx { continue; }
        if checksums_idx >= checksums.len() { break; }

        match sequence[i] {
            State::Working => {}
            _ => {
                let chunk_seq = sequence[i..=i + checksums[checksums_idx]].to_vec();
                next_idx = i + checksums[checksums_idx] + 1;

                // don't know when to advance to the next checksum

                let chunk_count = match cache.get(&(chunk_seq.len(), checksums[checksums_idx])) {
                    None => {
                        let c = match get_possible_sequences(&[], &chunk_seq, &[checksums[checksums_idx]]) {
                            None => {
                                checksums_idx += 1;

                                let chunk_seq = sequence[i..=i + checksums[checksums_idx]].to_vec();
                                next_idx = i + checksums[checksums_idx] + 1;
                                match cache.get(&(chunk_seq.len(), checksums[checksums_idx])) {
                                    None => get_possible_sequences(&[], &chunk_seq, &[checksums[checksums_idx]]).unwrap().len(),
                                    Some(c) => *c,
                                }
                            }
                            Some(c) => c.len(),
                        };
                        cache.insert((chunk_seq.len(), checksums[checksums_idx]), c);
                        c
                    }
                    Some(c) => *c,
                };

                count += chunk_count;
            }
        }
    }

    count
}

pub fn part_one(input: &str) -> Option<usize> {
    let dot_regex = Regex::new(r"\.+").unwrap();

    let lines: Vec<(Vec<State>, Vec<usize>)> = input.lines().map(|line| {
        let (first_part, second_part) = line.split(' ').tuples().next().unwrap();
        let first_part_normalized = dot_regex.replace_all(first_part, ".").into_owned();
        let first_part_normalized_2 = first_part_normalized.trim_start_matches('.');
        let first_part_normalized_3 = if !first_part_normalized_2.ends_with('.') { format!("{}.", first_part_normalized_2) } else { first_part_normalized_2.into() };
        let sequence: Vec<State> = first_part_normalized_3.chars().map(State::from).collect();
        let checksums: Vec<usize> = second_part.split(',').map(|c| c.parse::<usize>().unwrap()).collect();
        (sequence, checksums)
    }).collect();

    let result: usize = lines.iter().map(|(seq, checksums)| calc(seq, checksums)).sum();

    Some(result)
}

fn unfold_sequence(states: &Vec<State>) -> Vec<State> {
    states.iter().chain(iter::once(&State::Unknown)).cloned().cycle().take((states.len() + 1) * 2 - 1).collect()
}

fn unfold_checksum(checksum: &Vec<usize>) -> Vec<usize> {
    checksum.iter().cloned().cycle().take(checksum.len() * 2).collect()
}

pub fn part_two(input: &str) -> Option<usize> {
    let sequences: Vec<(Vec<State>, Vec<usize>)> = input.lines().map(|line| {
        let (first_part, second_part) = line.split(' ').tuples().next().unwrap();
        let init_seq: Vec<State> = first_part.chars().map(State::from).collect();
        let exp_seq: Vec<State> = unfold_sequence(&init_seq);
        let init_checksum: Vec<usize> = second_part.split(',').map(|c| c.parse::<usize>().unwrap()).collect();
        let exp_checksum: Vec<usize> = unfold_checksum(&init_checksum);
        (exp_seq, exp_checksum)
    }).collect();

    None
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
        assert_eq!(result, Some(525152));
    }
}
