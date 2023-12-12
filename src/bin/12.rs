use std::io::Read;
use std::iter;
use itertools::{Itertools, repeat_n, unfold};
use rayon::iter::{*};
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

fn get_possible_states(states: &[State]) -> Vec<Vec<State>> {
    let mut result: Vec<Vec<State>> = Vec::new();

    return match states.split_first() {
        None => result,
        Some((head, tail)) => {
            let new_tails = get_possible_states(tail);

            return match head {
                State::Unknown => {
                    if !new_tails.is_empty() {
                        for new_tail in new_tails {
                            result.push(iter::once(State::Working).chain(new_tail.clone()).collect());
                            result.push(iter::once(State::Damaged).chain(new_tail).collect());
                        }
                    } else {
                        result.push(vec!(State::Working));
                        result.push(vec!(State::Damaged));
                    }

                    result
                }
                h => {
                    if !new_tails.is_empty() {
                        for new_tail in new_tails {
                            result.push(iter::once(*h).chain(new_tail).collect());
                        }
                    } else {
                        result.push(vec!(*h));
                    }

                    result
                }
            };
        }
    };
}

fn get_possible_states_2<'a>(states: &'a [State]) -> Box<dyn Iterator<Item=Vec<State>> + 'a> {
    return match states.split_first() {
        None => Box::new(iter::empty()),
        Some((head, tail)) => {
            let mut new_tails = get_possible_states_2(tail).peekable();

            return match head {
                State::Unknown => {
                    if new_tails.peek().is_some() {
                        Box::new(new_tails.flat_map(|new_tail| {
                            let working = iter::once(State::Working).chain(new_tail.clone()).collect();
                            let damaged = iter::once(State::Damaged).chain(new_tail).collect();
                            vec!(working, damaged)
                        }))
                    } else {
                        Box::new(iter::once(vec!(State::Working)).chain(iter::once(vec!(State::Damaged))))
                    }
                }
                h => {
                    if new_tails.peek().is_some() {
                        Box::new(new_tails.flat_map(|new_tail| {
                            vec!(iter::once(*h).chain(new_tail).collect())
                        }))
                    } else {
                        Box::new(iter::once(vec!(*h)))
                    }
                }
            };
        }
    };
}

pub fn part_one(input: &str) -> Option<usize> {
    let lines: Vec<(Vec<State>, Vec<usize>)> = input.lines().map(|line| {
        let (first_part, second_part) = line.split(' ').tuples().next().unwrap();
        let states: Vec<State> = first_part.chars().map(State::from).collect();
        let checksum: Vec<usize> = second_part.split(',').map(|c| c.parse::<usize>().unwrap()).collect();
        (states, checksum)
    }).collect();

    let result: usize = lines.par_iter().map(|(states, checksum)| {
        let possible_sequences = get_possible_states_2(&states);

        let possible_counter = possible_sequences.filter(|sequence| {
            let mut calc_checksum: Vec<usize> = Vec::new();
            let mut counter: usize = 0;

            for state in sequence.iter() {
                match state {
                    State::Working => {
                        if counter > 0 { calc_checksum.push(counter); }
                        counter = 0;
                    }
                    State::Damaged => {
                        counter += 1;
                    }
                    State::Unknown => { panic!("unknown state encountered") }
                }
            }

            if counter > 0 { calc_checksum.push(counter); }

            &calc_checksum == checksum
        }).count();

        possible_counter
    }).sum();

    Some(result)
}

fn unfold_states(states: &Vec<State>) -> Vec<State> {
    states.iter().chain(iter::once(&State::Unknown)).cloned().cycle().take((states.len() + 1) * 5 - 1).collect()
}

fn unfold_checksum(checksum: &Vec<usize>) -> Vec<usize> {
    checksum.iter().cloned().cycle().take(checksum.len() * 5).collect()
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines: Vec<(Vec<State>, Vec<usize>)> = input.lines().map(|line| {
        let (first_part, second_part) = line.split(' ').tuples().next().unwrap();
        let init_states: Vec<State> = first_part.chars().map(State::from).collect();
        let states: Vec<State> = unfold_states(&init_states);
        let init_checksum: Vec<usize> = second_part.split(',').map(|c| c.parse::<usize>().unwrap()).collect();
        let checksum: Vec<usize> = unfold_checksum(&init_checksum);
        (states, checksum)
    }).collect();

    let result: usize = lines.par_iter().map(|(states, checksum)| {
        let possible_sequences = get_possible_states_2(&states);

        let possible_counter = possible_sequences.filter(|sequence| {
            let mut calc_checksum: Vec<usize> = Vec::new();
            let mut counter: usize = 0;

            for state in sequence.iter() {
                match state {
                    State::Working => {
                        if counter > 0 {
                            calc_checksum.push(counter);
                            if !checksum.starts_with(&calc_checksum) {
                                return false;
                            }
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

            &calc_checksum == checksum
        }).count();

        possible_counter
    }).sum();

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
