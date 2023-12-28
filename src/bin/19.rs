use std::collections::HashMap;
use std::str::FromStr;
use itertools::Itertools;
advent_of_code::solution!(19);

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let categories: HashMap<String, usize> = s[1..s.len() - 1].split(',').map(|c| {
            let (c_name, c_value) = c.split('=').tuples().next().unwrap();
            (c_name.to_string(), c_value.parse::<usize>().unwrap())
        }).collect();

        Ok(Part {
            x: categories["x"],
            m: categories["m"],
            a: categories["a"],
            s: categories["s"],
        })
    }
}

impl Part {
    fn get_category_value(&self, category: &str) -> usize {
        match category {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!("invalid category"),
        }
    }
}

type Transition = (Box<dyn Fn(&Part) -> bool>, String);

struct Workflow {
    name: String,
    transitions: Vec<Transition>,
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name_part, transitions_part) = s.split('{').tuples().next().unwrap();
        let transitions_str = transitions_part[0..transitions_part.len() - 1].split(',');

        let mut transitions: Vec<Transition> = vec!();

        transitions_str.for_each(|tr| {
            let parts: Vec<&str> = tr.split(':').collect();

            if parts.len() == 1 {
                let condition = Box::new(|_: &Part| -> bool { true });
                transitions.push((condition, parts.first().unwrap().to_string()));
            } else {
                let (condition_str, workflow) = parts.iter().tuples().next().unwrap();

                return if condition_str.contains('>') {
                    let (category_str, number_str) = condition_str.split('>').tuples().next().unwrap();
                    let category = category_str.to_owned();
                    let number = number_str.parse::<usize>().unwrap();
                    let condition = Box::new(move |part: &Part| -> bool { part.get_category_value(&category) > number });
                    transitions.push((condition, workflow.to_string()));
                } else {
                    let (category_str, number_str) = condition_str.split('<').tuples().next().unwrap();
                    let category = category_str.to_owned();
                    let number = number_str.parse::<usize>().unwrap();
                    let condition = Box::new(move |part: &Part| -> bool { part.get_category_value(&category) < number });
                    transitions.push((condition, workflow.to_string()));
                };
            }
        });

        Ok(Workflow { name: name_part.into(), transitions })
    }
}

impl Workflow {
    fn send_part(&self, part: &Part) -> String {
        for (condition, workflow) in &self.transitions {
            if condition(part) {
                return workflow.clone();
            }
        }

        panic!("no applying condition found")
    }
}

fn calc(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {

    let mut curr_workflow_name = "in".to_string();

    while curr_workflow_name != "A" && curr_workflow_name != "R" {
        curr_workflow_name = workflows[&curr_workflow_name].send_part(part);
    }

    curr_workflow_name == "A"
}

pub fn part_one(input: &str) -> Option<usize> {
    let (workflows_block, parts_block) = input.split("\n\n").tuples().next().unwrap();

    let parts = parts_block.lines().map(|p| Part::from_str(p).unwrap());
    let workflows: HashMap<String, Workflow> = workflows_block.lines().map(|w| {
        let workflow = Workflow::from_str(w).unwrap();
        (workflow.name.clone(), workflow)
    }).collect();

    let result = parts.filter(|part| calc(&part, &workflows)).map(|part| part.x + part.m + part.a + part.s).sum();

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
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
