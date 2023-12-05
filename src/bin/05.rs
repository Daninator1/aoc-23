use rayon::prelude::*;
advent_of_code::solution!(5);

#[derive(Debug)]
struct MappingGroup {
    mapping_values: Vec<MappingValues>,
}

impl MappingGroup {
    fn get_mapped_value(&self, value: u64) -> u64 {
        let mapping = self.mapping_values.iter().find(|mapping_value| {
            mapping_value.src_range_start <= value && mapping_value.src_range_start + mapping_value.range_length > value
        });

        match mapping {
            None => value,
            Some(m) => {
                m.dest_range_start + (value - m.src_range_start)
            }
        }
    }
}

#[derive(Debug)]
struct MappingValues {
    dest_range_start: u64,
    src_range_start: u64,
    range_length: u64,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let initial_seeds: Vec<u64> = lines
        .next().unwrap()
        .split(':')
        .last().unwrap()
        .trim()
        .split(' ')
        .map(|seed_str| seed_str.parse::<u64>().unwrap())
        .collect();

    let mapping_str_block = input
        .split("\n\n")
        .skip(1);

    let mapping_groups: Vec<MappingGroup> = mapping_str_block
        .map(|block| {
            let mappings = block
                .lines()
                .skip(1)
                .map(|line| {
                    let map_values: Vec<u64> = line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
                    if map_values.len() != 3 { panic!("invalid amount of mapping values") };
                    MappingValues { dest_range_start: map_values[0], src_range_start: map_values[1], range_length: map_values[2] }
                })
                .collect();

            MappingGroup { mapping_values: mappings }
        })
        .collect();

    let results = initial_seeds.iter().map(|initial_seed| {
        let mut curr_value: u64 = *initial_seed;

        for mapping_group in &mapping_groups {
            curr_value = mapping_group.get_mapped_value(curr_value);
        }

        (*initial_seed, curr_value)
    });

    Some(results.min_by_key(|x| x.1).unwrap().1)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let initial_seeds: Vec<u64> = lines
        .next().unwrap()
        .split(':')
        .last().unwrap()
        .trim()
        .split(' ')
        .map(|seed_str| seed_str.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .chunks(2)
        .flat_map(|chunk| {
            (0..chunk[1]).map(|i| chunk[0] + i)
        })
        .collect();

    let mapping_str_block = input
        .split("\n\n")
        .skip(1);

    let mapping_groups: Vec<MappingGroup> = mapping_str_block
        .map(|block| {
            let mappings = block
                .lines()
                .skip(1)
                .map(|line| {
                    let map_values: Vec<u64> = line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
                    if map_values.len() != 3 { panic!("invalid amount of mapping values") };
                    MappingValues { dest_range_start: map_values[0], src_range_start: map_values[1], range_length: map_values[2] }
                })
                .collect();

            MappingGroup { mapping_values: mappings }
        })
        .collect();

    let results = initial_seeds.into_par_iter().map(|initial_seed| {
        let mut curr_value: u64 = initial_seed;

        for mapping_group in &mapping_groups {
            curr_value = mapping_group.get_mapped_value(curr_value);
        }

        (initial_seed, curr_value)
    });

    Some(results.min_by_key(|x| x.1).unwrap().1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
