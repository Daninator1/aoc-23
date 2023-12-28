use std::cmp::{max, min, Ordering};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
use std::ops::Range;
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

#[derive(Debug)]
struct HypPart {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
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

type Transition = (Box<dyn Fn(&Part) -> bool + Send + Sync>, (String, Ordering, usize), String);

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
                transitions.push((condition, ("x".into(), Equal, 0), parts.first().unwrap().to_string()));
            } else {
                let (condition_str, workflow) = parts.iter().tuples().next().unwrap();

                return if condition_str.contains('>') {
                    let (category_str, number_str) = condition_str.split('>').tuples().next().unwrap();
                    let category = category_str.to_owned();
                    let category_2 = category.clone();
                    let number = number_str.parse::<usize>().unwrap();
                    let condition = Box::new(move |part: &Part| -> bool { part.get_category_value(&category) > number });
                    transitions.push((condition, (category_2, Greater, number), workflow.to_string()));
                } else {
                    let (category_str, number_str) = condition_str.split('<').tuples().next().unwrap();
                    let category = category_str.to_owned();
                    let category_2 = category.clone();
                    let number = number_str.parse::<usize>().unwrap();
                    let condition = Box::new(move |part: &Part| -> bool { part.get_category_value(&category) < number });
                    transitions.push((condition, (category_2, Less, number), workflow.to_string()));
                };
            }
        });

        Ok(Workflow { name: name_part.into(), transitions })
    }
}

impl Workflow {
    fn send_part(&self, part: &Part) -> String {
        for (condition, _, workflow) in &self.transitions {
            if condition(part) {
                return workflow.clone();
            }
        }

        panic!("no applying condition found")
    }

    fn send_hyp_part(&self, hyp_part: &HypPart) -> Vec<(String, HypPart)> {
        let mut result = vec!();

        let mut curr_hyp_part: HypPart = HypPart {
            x: Range { start: hyp_part.x.start, end: hyp_part.x.end },
            m: Range { start: hyp_part.m.start, end: hyp_part.m.end },
            a: Range { start: hyp_part.a.start, end: hyp_part.a.end },
            s: Range { start: hyp_part.s.start, end: hyp_part.s.end },
        };

        for (_, (category, ordering, number), workflow) in &self.transitions {
            match ordering {
                Less => {
                    match &category[..] {
                        "x" => {
                            if curr_hyp_part.x.start < *number {
                                result.push((workflow.clone(), HypPart {
                                    x: Range { start: curr_hyp_part.x.start, end: number - 1 },
                                    m: curr_hyp_part.m.clone(),
                                    a: curr_hyp_part.a.clone(),
                                    s: curr_hyp_part.s.clone(),
                                }));
                            }

                            curr_hyp_part = HypPart {
                                x: Range { start: *number, end: curr_hyp_part.x.end },
                                m: curr_hyp_part.m.clone(),
                                a: curr_hyp_part.a.clone(),
                                s: curr_hyp_part.s.clone(),
                            }
                        }
                        "m" => {
                            if curr_hyp_part.m.start < *number {
                                result.push((workflow.clone(), HypPart {
                                    x: curr_hyp_part.x.clone(),
                                    m: Range { start: curr_hyp_part.m.start, end: number - 1 },
                                    a: curr_hyp_part.a.clone(),
                                    s: curr_hyp_part.s.clone(),
                                }));
                            }

                            curr_hyp_part = HypPart {
                                x: curr_hyp_part.x.clone(),
                                m: Range { start: *number, end: curr_hyp_part.m.end },
                                a: curr_hyp_part.a.clone(),
                                s: curr_hyp_part.s.clone(),
                            }
                        }
                        "a" => {
                            if curr_hyp_part.a.start < *number {
                                result.push((workflow.clone(), HypPart {
                                    x: curr_hyp_part.x.clone(),
                                    m: curr_hyp_part.m.clone(),
                                    a: Range { start: curr_hyp_part.a.start, end: number - 1 },
                                    s: curr_hyp_part.s.clone(),
                                }));
                            }

                            curr_hyp_part = HypPart {
                                x: curr_hyp_part.x.clone(),
                                m: curr_hyp_part.m.clone(),
                                a: Range { start: *number, end: curr_hyp_part.a.end },
                                s: curr_hyp_part.s.clone(),
                            }
                        }
                        "s" => {
                            if curr_hyp_part.s.start < *number {
                                result.push((workflow.clone(), HypPart {
                                    x: curr_hyp_part.x.clone(),
                                    m: curr_hyp_part.m.clone(),
                                    a: curr_hyp_part.a.clone(),
                                    s: Range { start: curr_hyp_part.s.start, end: number - 1 },
                                }));
                            }

                            curr_hyp_part = HypPart {
                                x: curr_hyp_part.x.clone(),
                                m: curr_hyp_part.m.clone(),
                                a: curr_hyp_part.a.clone(),
                                s: Range { start: *number, end: curr_hyp_part.s.end },
                            }
                        }
                        _ => panic!("invalid category"),
                    }
                }
                Greater => {
                    match &category[..] {
                        "x" => {
                            if curr_hyp_part.x.end > *number {
                                result.push((workflow.clone(), HypPart {
                                    x: Range { start: number + 1, end: curr_hyp_part.x.end },
                                    m: curr_hyp_part.m.clone(),
                                    a: curr_hyp_part.a.clone(),
                                    s: curr_hyp_part.s.clone(),
                                }));
                            }

                            curr_hyp_part = HypPart {
                                x: Range { start: curr_hyp_part.x.start, end: *number },
                                m: curr_hyp_part.m.clone(),
                                a: curr_hyp_part.a.clone(),
                                s: curr_hyp_part.s.clone(),
                            }
                        }
                        "m" => {
                            if curr_hyp_part.m.end > *number {
                                result.push((workflow.clone(), HypPart {
                                    x: curr_hyp_part.x.clone(),
                                    m: Range { start: number + 1, end: curr_hyp_part.m.end },
                                    a: curr_hyp_part.a.clone(),
                                    s: curr_hyp_part.s.clone(),
                                }));
                            }

                            curr_hyp_part = HypPart {
                                x: curr_hyp_part.x.clone(),
                                m: Range { start: curr_hyp_part.m.start, end: *number },
                                a: curr_hyp_part.a.clone(),
                                s: curr_hyp_part.s.clone(),
                            }
                        }
                        "a" => {
                            if curr_hyp_part.a.end > *number {
                                result.push((workflow.clone(), HypPart {
                                    x: curr_hyp_part.x.clone(),
                                    m: curr_hyp_part.m.clone(),
                                    a: Range { start: number + 1, end: curr_hyp_part.a.end },
                                    s: curr_hyp_part.s.clone(),
                                }));
                            }

                            curr_hyp_part = HypPart {
                                x: curr_hyp_part.x.clone(),
                                m: curr_hyp_part.m.clone(),
                                a: Range { start: curr_hyp_part.a.start, end: *number },
                                s: curr_hyp_part.s.clone(),
                            }
                        }
                        "s" => {
                            if curr_hyp_part.s.end > *number {
                                result.push((workflow.clone(), HypPart {
                                    x: curr_hyp_part.x.clone(),
                                    m: curr_hyp_part.m.clone(),
                                    a: curr_hyp_part.a.clone(),
                                    s: Range { start: number + 1, end: curr_hyp_part.s.end },
                                }));
                            }

                            curr_hyp_part = HypPart {
                                x: curr_hyp_part.x.clone(),
                                m: curr_hyp_part.m.clone(),
                                a: curr_hyp_part.a.clone(),
                                s: Range { start: curr_hyp_part.s.start, end: *number },
                            }
                        }
                        _ => panic!("invalid category"),
                    }
                }
                Equal => {
                    result.push((workflow.clone(), HypPart {
                        x: curr_hyp_part.x.clone(),
                        m: curr_hyp_part.m.clone(),
                        a: curr_hyp_part.a.clone(),
                        s: curr_hyp_part.s.clone(),
                    }));
                }
            }
        }

        result
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

    let result = parts.filter(|part| calc(part, &workflows)).map(|part| part.x + part.m + part.a + part.s).sum();

    Some(result)
}

fn calc_2(workflows: &HashMap<String, Workflow>) -> Vec<HypPart> {
    let hyp_part = HypPart {
        x: Range { start: 1, end: 4000 },
        m: Range { start: 1, end: 4000 },
        a: Range { start: 1, end: 4000 },
        s: Range { start: 1, end: 4000 },
    };

    let mut accepted: Vec<HypPart> = vec!();

    let mut curr = vec!(("in".to_string(), hyp_part));

    while !curr.is_empty() {
        let res = curr.iter().flat_map(|c| workflows[&c.0].send_hyp_part(&c.1));
        let mut next: Vec<(String, HypPart)> = vec!();

        for r in res {
            match r.0.as_str() {
                "A" => { accepted.push(r.1); }
                "R" => {}
                _ => { next.push(r) }
            }
        }

        curr = next;
    }

    accepted
}

fn get_overlap(me: &HypPart, other: &HypPart) -> usize {
    let max_start_x = max(me.x.start, other.x.start);
    let min_end_x = min(me.x.end, other.x.end);
    let overlap_x = if min_end_x >= max_start_x { min_end_x - max_start_x + 1 } else { 0 };

    let max_start_m = max(me.m.start, other.m.start);
    let min_end_m = min(me.m.end, other.m.end);
    let overlap_m = if min_end_m >= max_start_m { min_end_m - max_start_m + 1 } else { 0 };

    let max_start_a = max(me.a.start, other.a.start);
    let min_end_a = min(me.a.end, other.a.end);
    let overlap_a = if min_end_a >= max_start_a { min_end_a - max_start_a + 1 } else { 0 };

    let max_start_s = max(me.s.start, other.s.start);
    let min_end_s = min(me.s.end, other.s.end);
    let overlap_s = if min_end_s >= max_start_s { min_end_s - max_start_s + 1 } else { 0 };

    overlap_x * overlap_m * overlap_a * overlap_s
}

pub fn part_two(input: &str) -> Option<usize> {
    let (workflows_block, _) = input.split("\n\n").tuples().next().unwrap();

    let workflows: HashMap<String, Workflow> = workflows_block.lines().map(|w| {
        let workflow = Workflow::from_str(w).unwrap();
        (workflow.name.clone(), workflow)
    }).collect();

    let hyp_results = calc_2(&workflows);

    let mut visited_hyp_results: Vec<&HypPart> = vec!();

    let mut combinations = 0;

    for hyp_result in &hyp_results {
        let x_comb = hyp_result.x.end - hyp_result.x.start + 1;
        let m_comb = hyp_result.m.end - hyp_result.m.start + 1;
        let a_comb = hyp_result.a.end - hyp_result.a.start + 1;
        let s_comb = hyp_result.s.end - hyp_result.s.start + 1;
        let comb = x_comb * m_comb * a_comb * s_comb;

        let mut deduct = 0;

        for visited in &visited_hyp_results {
            let overlap = get_overlap(hyp_result, visited);
            deduct += overlap;
        }

        deduct = min(deduct, comb);

        combinations += comb - deduct;
        visited_hyp_results.push(hyp_result);
    }

    Some(combinations)
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
        assert_eq!(result, Some(167409079868000));
    }
}
