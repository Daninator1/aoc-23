use itertools::Itertools;
use rand::Rng;
advent_of_code::solution!(25);

#[derive(Debug, Clone)]
struct Link {
    id: String,
    from: String,
    to: String,
}

fn create_links(line: &str) -> Vec<Link> {
    let (from, to_part) = line.split(": ").tuples().next().unwrap();

    to_part.split(' ').map(|to| {
        Link { id: from.to_string() + to, from: from.to_string(), to: to.to_string() }
    }).collect()
}

fn get_next(current: &String, links: &[Link], visited: &[String]) -> Vec<String> {
    links.iter().filter_map(|link| {
        if &link.from == current && !visited.contains(&link.to) {
            return Some(link.to.clone());
        }

        if &link.to == current && !visited.contains(&link.from) {
            return Some(link.from.clone());
        }

        None
    }).collect()
}

fn get_group(start: String, links: &[Link]) -> Vec<String> {
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

fn find_groups<const AMOUNT: usize>(links: &[Link]) -> Option<Vec<Vec<String>>> {
    let mut start = links[0].from.clone();
    let mut groups: Vec<Vec<String>> = vec!();

    loop {
        if groups.len() == AMOUNT { return None; }

        let group = get_group(start, links);

        groups.push(group);

        let next_start = links.iter().find_map(|link| {
            if !groups.iter().flatten().any(|g| g == &link.from) {
                return Some(&link.from);
            }

            if !groups.iter().flatten().any(|g| g == &link.to) {
                return Some(&link.to);
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

fn are_all_same_nodes(links: &[Link]) -> bool {
    let compare_link = links[0].clone();

    for link in links.iter().skip(1) {
        if !((link.from == compare_link.from && link.to == compare_link.to) || (link.from == compare_link.to && link.to == compare_link.from)) {
            return false;
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<usize> {
    let links: Vec<_> = input.lines().flat_map(create_links).collect();

    let mut rng = rand::thread_rng();

    let mut current_links = links.clone();

    while current_links.len() != 3 {
        current_links = links.clone();

        while !are_all_same_nodes(&current_links) {
            let index = rng.gen_range(0..current_links.len());

            let link = current_links[index].clone();

            current_links = current_links.iter().filter_map(|l| {
                if l.from == link.to {
                    if link.from == l.to { return None; }
                    return Some(Link { id: l.id.clone(), from: link.from.clone(), to: l.to.clone() });
                }
                if l.to == link.to {
                    if link.from == l.from { return None; }
                    return Some(Link { id: l.id.clone(), from: l.from.clone(), to: link.from.clone() });
                }

                Some(l.clone())
            }).collect();
        }
    }

    let disconnects = current_links.clone();

    let groups = find_groups::<2>(&links.clone().into_iter().filter(|link| !disconnects.iter().any(|d| d.id == link.id)).collect::<Vec<_>>());
    return groups.map(|g| {
        g.iter().fold(1, |acc, group| acc * group.len())
    });
}

pub fn part_two(_input: &str) -> Option<u32> {
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
