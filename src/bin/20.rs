use std::collections::HashMap;
use std::fmt::Debug;
use std::str::Lines;
use itertools::Itertools;
advent_of_code::solution!(20);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Pulse {
    High,
    Low,
}

trait Module: Debug {
    fn id(&self) -> &String;

    fn process(&mut self, input: (String, Pulse)) -> Vec<(String, String, Pulse)>;
}

#[derive(Debug)]
struct BroadcastModule {
    id: String,
    outputs: Vec<String>,
}

impl BroadcastModule {
    fn new(id: String, outputs: Vec<String>) -> Self {
        BroadcastModule { id, outputs }
    }
}

impl Module for BroadcastModule {
    fn id(&self) -> &String {
        &self.id
    }

    fn process(&mut self, input: (String, Pulse)) -> Vec<(String, String, Pulse)> {
        self.outputs.iter().map(|o| {
            println!("{:?} -{:?} -> {:?}", self.id, input.1, o);
            (self.id().clone(), o.clone(), input.1)
        }).collect()
    }
}

#[derive(Debug)]
struct FlipFlopModule {
    id: String,
    outputs: Vec<String>,
    enabled: bool,
}

impl FlipFlopModule {
    fn new(id: String, outputs: Vec<String>) -> Self {
        FlipFlopModule { id, outputs, enabled: false }
    }
}

impl Module for FlipFlopModule {
    fn id(&self) -> &String {
        &self.id
    }

    fn process(&mut self, input: (String, Pulse)) -> Vec<(String, String, Pulse)> {
        let id = self.id().clone();

        if input.1 == Pulse::High { return vec!(); }

        let result = self.outputs.iter().map(|o| {
            match self.enabled {
                true => {
                    println!("{:?} -{:?} -> {:?}", self.id, Pulse::Low, o);
                    (id.clone(), o.clone(), Pulse::Low)
                }
                false => {
                    println!("{:?} -{:?} -> {:?}", self.id, Pulse::High, o);
                    (id.clone(), o.clone(), Pulse::High)
                }
            }
        }).collect();

        self.enabled = !self.enabled;
        result
    }
}

#[derive(Debug)]
struct ConjunctionModule {
    id: String,
    outputs: Vec<String>,
    last_pulses: HashMap<String, Pulse>,
}

impl ConjunctionModule {
    fn new(id: String, input_ids: Vec<String>, outputs: Vec<String>) -> Self {
        ConjunctionModule { id, outputs, last_pulses: input_ids.iter().map(|i| (i.clone(), Pulse::Low)).collect() }
    }
}

impl Module for ConjunctionModule {
    fn id(&self) -> &String {
        &self.id
    }

    fn process(&mut self, input: (String, Pulse)) -> Vec<(String, String, Pulse)> {
        let id = self.id().clone();

        self.last_pulses.insert(input.0.clone(), input.1);

        self.outputs.iter().map(|o| {
            if self.last_pulses.iter().all(|l| l.1 == &Pulse::High) {
                println!("{:?} -{:?} -> {:?}", self.id, Pulse::Low, o);
                (id.clone(), o.clone(), Pulse::Low)
            } else {
                println!("{:?} -{:?} -> {:?}", self.id, Pulse::High, o);
                (id.clone(), o.clone(), Pulse::High)
            }
        }).collect()
    }
}

fn create_module(line: &str, all_lines: &Lines) -> Box<dyn Module> {
    let (id_part, output_part) = line.split(" -> ").tuples().next().unwrap();

    let output_ids = output_part.split(", ");

    if id_part.starts_with('%') {
        return Box::new(FlipFlopModule::new(id_part[1..id_part.len()].into(), output_ids.map(|o| o.to_string()).collect()));
    }

    if id_part.starts_with('&') {
        let other_ids = all_lines.clone().filter_map(|l| {
            let (other_id_part, other_output_part) = l.split(" -> ").tuples().next().unwrap();

            if other_output_part.contains(&id_part[1..id_part.len()]) {
                Some(other_id_part[1..other_id_part.len()].to_string())
            } else {
                None
            }
        });

        return Box::new(ConjunctionModule::new(id_part[1..id_part.len()].into(), other_ids.collect(), output_ids.map(|o| o.to_string()).collect()));
    }

    Box::new(BroadcastModule::new(id_part.into(), output_ids.map(|o| o.to_string()).collect()))
}

fn run(modules: &mut HashMap<String, Box<dyn Module>>) -> (usize, usize) {
    let broadcaster_module = modules.get_mut("broadcaster").unwrap();

    let mut curr_result = broadcaster_module.process(("button".into(), Pulse::Low));

    let mut low_counter = 0;
    let mut high_counter = 0;

    while !curr_result.is_empty() {
        let mut next_results = vec!();

        for (from, to, pulse) in &curr_result {
            match pulse {
                Pulse::High => { high_counter += 1; }
                Pulse::Low => { low_counter += 1; }
            }
            if let Some(next_module) = modules.get_mut(to) {
                next_results.push(next_module.process((from.clone(), *pulse)));
            }
        }

        curr_result = next_results.iter().flatten().cloned().collect();
    }

    (low_counter + 1, high_counter)
}

pub fn part_one(input: &str) -> Option<usize> {
    let all_lines = input.lines();

    let mut modules: HashMap<_, _> = input.lines().map(|l| {
        let module = create_module(l, &all_lines);
        (module.id().clone(), module)
    }).collect();

    let result = (0..1000)
        .map(|_| run(&mut modules))
        .fold((0, 0), |acc, curr| (acc.0 + curr.0, acc.1 + curr.1));

    dbg!(&result);

    Some(result.0 * result.1)
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
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
