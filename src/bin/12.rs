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

fn get_possible_states_3(curr_states: &[State], rem_states: &[State], checksum: &[usize]) -> Option<Vec<Vec<State>>> {
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

                let working = get_possible_states_3(curr_states.iter().chain(iter::once(&State::Working)).cloned().collect::<Vec<State>>().as_slice(), &rem_states[1..], checksum);
                if let Some(w) = working {
                    w.into_iter().for_each(|seq| result.push(seq));
                }

                let damaged = get_possible_states_3(curr_states.iter().chain(iter::once(&State::Damaged)).cloned().collect::<Vec<State>>().as_slice(), &rem_states[1..], checksum);
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
                get_possible_states_3(curr_states.iter().chain(iter::once(&known)).cloned().collect::<Vec<State>>().as_slice(), &rem_states[1..], checksum)
            }
        };
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let lines: Vec<(Vec<State>, Vec<usize>)> = input.lines().map(|line| {
        let (first_part, second_part) = line.split(' ').tuples().next().unwrap();
        let states: Vec<State> = first_part.chars().map(State::from).collect();
        let checksum: Vec<usize> = second_part.split(',').map(|c| c.parse::<usize>().unwrap()).collect();
        (states, checksum)
    }).collect();

    let result: usize = lines.par_iter().map(|(states, checksum)| {
        let possible_sequences = get_possible_states_3(&[], states, checksum);
        match possible_sequences {
            None => 0,
            Some(s) => {
                return s.len();
            }
        }
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
        let possible_sequences = get_possible_states_3(&[], states, checksum);
        match possible_sequences {
            None => 0,
            Some(s) => {
                return s.len();
            }
        }
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
