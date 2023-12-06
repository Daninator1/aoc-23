use regex::Regex;
advent_of_code::solution!(6);

fn parse_races(input: &str) -> Vec<(u32, u32)> {
    let whitespace_regex = Regex::new(r"\s+").unwrap();
    let numbers: Vec<Vec<u32>> = input.lines().map(|line| {
        let numbers_str = line.split(':').last().unwrap().trim();
        let normalized_numbers_str = whitespace_regex.replace_all(numbers_str, " ").into_owned();
        normalized_numbers_str.split(' ').map(|number_str| number_str.parse::<u32>().unwrap()).collect()
    }).collect();

    let times = numbers.first().unwrap().iter();
    let distances = numbers.last().unwrap().iter();

    times.cloned().zip(distances.cloned()).collect()
}

fn parse_races_alt(input: &str) -> (usize, usize) {
    let numbers: Vec<usize> = input.lines().map(|line| {
        let number_str = line.split(':').last().unwrap().replace(' ', "");
        number_str.parse::<usize>().unwrap()
    }).collect();

    let time = *numbers.first().unwrap();
    let distance = *numbers.last().unwrap();

    (time, distance)
}

fn calc_distance(time_held: u32, time: u32) -> u32 {
    let rem_time = time - time_held;
    time_held * rem_time
}

fn calc_distance_alt(time_held: usize, time: usize) -> usize {
    let rem_time = time - time_held;
    time_held * rem_time
}

pub fn part_one(input: &str) -> Option<usize> {
    let races = parse_races(input);
    let result = races.iter().map(|race| {
        let dist_reached = (0..=race.0).map(|time_held| calc_distance(time_held, race.0));
        dist_reached.filter(|dist| dist > &race.1).count()
    }).product::<usize>();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let race = parse_races_alt(input);
    let dist_reached = (0..=race.0).map(|time_held| calc_distance_alt(time_held, race.0));
    let result = dist_reached.filter(|dist| *dist > race.1).count();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
