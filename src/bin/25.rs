use itertools::Itertools;
use rayon::prelude::{*};
advent_of_code::solution!(25);

fn create_links(line: &str) -> Vec<(String, String)> {
    let (from, to_part) = line.split(": ").tuples().next().unwrap();

    to_part.split(' ').map(|to| {
        (from.to_string(), to.to_string())
    }).collect()
}

fn get_next(current: &String, links: &[(String, String)], visited: &Vec<String>) -> Vec<String> {
    links.iter().filter_map(|link| {
        if &link.0 == current && !visited.contains(&link.1) {
            return Some(link.1.clone());
        }

        if &link.1 == current && !visited.contains(&link.0) {
            return Some(link.0.clone());
        }

        None
    }).collect()
}

fn get_group(start: String, links: &[(String, String)]) -> Vec<String> {
    let mut current = vec!(start);
    let mut visited = current.clone();

    while !current.is_empty() {
        current = current.iter().flat_map(|c| {
            let next = get_next(c, links, &visited);
            for n in &next {
                visited.push(n.clone());
            }
            next
        }).collect()
    }

    visited
}

fn find_groups<const AMOUNT: usize>(links: &Vec<(String, String)>) -> Option<Vec<Vec<String>>> {
    let mut start = links[0].0.clone();
    let mut groups: Vec<Vec<String>> = vec!();

    loop {
        if groups.len() == AMOUNT { return None; }

        let group = get_group(start, &links);

        groups.push(group);

        let next_start = links.iter().find_map(|link| {
            if !groups.iter().flatten().any(|g| g == &link.0) {
                return Some(&link.0);
            }

            if !groups.iter().flatten().any(|g| g == &link.1) {
                return Some(&link.1);
            }

            None
        });

        match next_start {
            None => { break; }
            Some(x) => { start = x.clone(); }
        }
    }

    if groups.len() != AMOUNT { return None; }

    Some(groups)
}

pub fn part_one(input: &str) -> Option<usize> {
    let links: Vec<_> = input.lines().flat_map(create_links).collect();

    let mut disconnects = links.iter().combinations(3);

    disconnects.find_map(|disconnect| {
        let groups = find_groups::<2>(&links.clone().into_iter().filter(|link| !disconnect.contains(&link)).collect());
        return groups.map(|g| {
            g.iter().fold(1, |acc, group| acc * group.len())
        });
    })
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
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
