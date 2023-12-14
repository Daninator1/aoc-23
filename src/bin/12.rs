use std::io::Read;
use std::iter;
use itertools::{Itertools, izip, repeat_n, unfold};
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
                            result.push(iter::once(State::Damaged).chain(new_tail.clone()).collect());
                            result.push(iter::once(State::Working).chain(new_tail).collect());
                        }
                    } else {
                        result.push(vec!(State::Damaged));
                        result.push(vec!(State::Working));
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
        let possible_sequences = get_possible_states(&states);

        let possible_counter = possible_sequences.iter().filter(|sequence| {
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

fn unfold_sequence(states: &Vec<State>) -> Vec<State> {
    states.iter().chain(iter::once(&State::Unknown)).cloned().cycle().take((states.len() + 1) * 2 - 1).collect()
}

fn unfold_checksum(checksum: &Vec<usize>) -> Vec<usize> {
    checksum.iter().cloned().cycle().take(checksum.len() * 2).collect()
}

fn calculate_checksum(seq: &Vec<State>) -> Vec<usize> {
    let mut calc_checksum: Vec<usize> = Vec::new();
    let mut counter: usize = 0;

    for poss_state in seq.iter() {
        match poss_state {
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

    calc_checksum
}

fn is_checksum_possible(calc_checksum: &Vec<usize>, checksum: &Vec<usize>, rem_length: usize) -> bool {
    checksum.starts_with(calc_checksum.split_last().unwrap().1) &&
        calc_checksum.len() <= checksum.len() &&
        calc_checksum.last().unwrap() <= &checksum[calc_checksum.len() - 1] &&
        checksum[calc_checksum.len() - 1] - calc_checksum.last().unwrap() <= rem_length
}

fn get_possible_sequences_exp<'a>(sequences: &'a Vec<Vec<State>>, filler_sequences: &'a Vec<Vec<State>>, checksum: &'a Vec<usize>) -> Box<dyn Iterator<Item=Vec<State>> + Send + Sync + 'a> {
    let s1s = &sequences;
    let f1s = &filler_sequences;
    let s2s = &sequences;
    let f2s = &filler_sequences;
    let s3s = &sequences;
    let f3s = &filler_sequences;
    let s4s = &sequences;
    let f4s = &filler_sequences;
    let s5s = &sequences;

    let sers = s1s.iter().flat_map(|s1| {
        let curr_seq = [s1.as_slice()].concat();
        let calc_checksum: Vec<usize> = calculate_checksum(&curr_seq);
        let rem_length: usize = curr_seq.len() + 4;

        if !is_checksum_possible(&calc_checksum, checksum, rem_length) {
            return None;
        }

        Some(f1s.iter().flat_map(|f1| {
            let curr_seq = [s1.as_slice(), f1.as_slice()].concat();
            let calc_checksum: Vec<usize> = calculate_checksum(&curr_seq);
            let rem_length: usize = curr_seq.len() + 3;

            if !is_checksum_possible(&calc_checksum, checksum, rem_length) {
                return None;
            }

            Some(s2s.iter().flat_map(|s2| {
                f2s.iter().flat_map(|f2| {
                    s3s.iter().flat_map(|s3| {
                        f3s.iter().flat_map(|f3| {
                            s4s.iter().flat_map(|s4| {
                                f4s.iter().flat_map(|f4| {
                                    s5s.iter().filter_map(|s5| {
                                        let curr_seq = [s1.as_slice(), f1.as_slice(),
                                            s2.as_slice(), f2.as_slice(),
                                            s3.as_slice(), f3.as_slice(),
                                            s4.as_slice(), f4.as_slice(),
                                            s5.as_slice()].concat();
                                        if calculate_checksum(&curr_seq) == checksum.clone() {
                                            Some(curr_seq)
                                        } else {
                                            None
                                        }
                                    })
                                })
                            })
                        })
                    })
                })
            }))
        }).flatten())
    }).flatten();

    Box::new(sers)
}

fn get_possible_sequences_exp_2(sequences: Vec<Vec<State>>, filler_sequences: Vec<Vec<State>>, checksum: &Vec<usize>) -> Vec<Vec<State>> {
    let s1s = &sequences;
    let f1s = &filler_sequences;
    let s2s = &sequences;
    let f2s = &filler_sequences;
    let s3s = &sequences;
    let f3s = &filler_sequences;
    let s4s = &sequences;
    let f4s = &filler_sequences;
    let s5s = &sequences;

    let mut result: Vec<Vec<State>> = Vec::new();

    for s1 in s1s {
        let curr_seq = [s1.as_slice()].concat();
        let calc_checksum: Vec<usize> = calculate_checksum(&curr_seq);
        let rem_length: usize = curr_seq.len() + 4;

        if is_checksum_possible(&calc_checksum, checksum, rem_length) {
            for f1 in f1s {
                let curr_seq = [s1.as_slice(), f1.as_slice()].concat();
                let calc_checksum: Vec<usize> = calculate_checksum(&curr_seq);
                let rem_length: usize = curr_seq.len() + 3;

                if is_checksum_possible(&calc_checksum, checksum, rem_length) {
                    for s2 in s2s {
                        let curr_seq = [s1.as_slice(), f1.as_slice(), s2.as_slice()].concat();
                        let calc_checksum: Vec<usize> = calculate_checksum(&curr_seq);
                        let rem_length: usize = curr_seq.len() + 3;

                        if is_checksum_possible(&calc_checksum, checksum, rem_length) {
                            for f2 in f2s {
                                let curr_seq = [s1.as_slice(), f1.as_slice(), s2.as_slice(), f2.as_slice()].concat();
                                let calc_checksum: Vec<usize> = calculate_checksum(&curr_seq);
                                let rem_length: usize = curr_seq.len() + 2;

                                if is_checksum_possible(&calc_checksum, checksum, rem_length) {
                                    for s3 in s3s {
                                        let curr_seq = [s1.as_slice(), f1.as_slice(), s2.as_slice(), f2.as_slice(), s3.as_slice()].concat();
                                        let calc_checksum: Vec<usize> = calculate_checksum(&curr_seq);
                                        let rem_length: usize = curr_seq.len() + 2;

                                        if is_checksum_possible(&calc_checksum, checksum, rem_length) {
                                            for f3 in f3s {
                                                let curr_seq = [s1.as_slice(), f1.as_slice(), s2.as_slice(), f2.as_slice(), s3.as_slice(), f3.as_slice()].concat();
                                                let calc_checksum: Vec<usize> = calculate_checksum(&curr_seq);
                                                let rem_length: usize = curr_seq.len() + 1;

                                                if is_checksum_possible(&calc_checksum, checksum, rem_length) {
                                                    for s4 in s4s {
                                                        let curr_seq = [s1.as_slice(), f1.as_slice(), s2.as_slice(), f2.as_slice(), s3.as_slice(), f3.as_slice(), s4.as_slice()].concat();
                                                        let calc_checksum: Vec<usize> = calculate_checksum(&curr_seq);
                                                        let rem_length: usize = curr_seq.len() + 1;

                                                        if is_checksum_possible(&calc_checksum, checksum, rem_length) {
                                                            for f4 in f4s {
                                                                let curr_seq = [s1.as_slice(), f1.as_slice(), s2.as_slice(), f2.as_slice(), s3.as_slice(), f3.as_slice(), s4.as_slice(), f4.as_slice()].concat();
                                                                let calc_checksum: Vec<usize> = calculate_checksum(&curr_seq);
                                                                let rem_length: usize = curr_seq.len();

                                                                if is_checksum_possible(&calc_checksum, checksum, rem_length) {
                                                                    for s5 in s5s {
                                                                        let curr_seq = [s1.as_slice(), f1.as_slice(),
                                                                            s2.as_slice(), f2.as_slice(),
                                                                            s3.as_slice(), f3.as_slice(),
                                                                            s4.as_slice(), f4.as_slice(),
                                                                            s5.as_slice()].concat();
                                                                        if calculate_checksum(&curr_seq) == checksum.clone() {
                                                                            result.push(curr_seq);
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    result
}

pub fn part_two(input: &str) -> Option<usize> {
    let sequences: Vec<((Vec<State>, Vec<usize>), (Vec<State>, Vec<usize>))> = input.lines().map(|line| {
        let (first_part, second_part) = line.split(' ').tuples().next().unwrap();
        let init_seq: Vec<State> = first_part.chars().map(State::from).collect();
        let exp_seq: Vec<State> = unfold_sequence(&init_seq);
        let init_checksum: Vec<usize> = second_part.split(',').map(|c| c.parse::<usize>().unwrap()).collect();
        let exp_checksum: Vec<usize> = unfold_checksum(&init_checksum);
        ((init_seq, init_checksum), (exp_seq, exp_checksum))
    }).collect();

    let result = sequences.par_iter().map(|(init_seq, exp_seq)| {
        let first = get_possible_states_3(&[], &init_seq.0, &init_seq.1).unwrap().len();
        let second = get_possible_states_3(&[], &exp_seq.0, &exp_seq.1).unwrap().len();
        if second % first > 0 { panic!("{:?}", &init_seq) }
        first * ((second / first).pow(5 - 1))
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
