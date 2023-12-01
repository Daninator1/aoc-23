advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let filtered_string = line
                .chars()
                .filter(|char| char.is_ascii_digit())
                .collect::<String>();

            let first = filtered_string.chars().next().unwrap();
            let last = filtered_string.chars().last().unwrap();

            [first, last]
                .iter()
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        }
        )
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(convert)
        .sum::<u32>()
        .into()
}

fn convert(line: &str) -> u32 {
    let numbers = [
        ("one", 1), ("1", 1),
        ("two", 2), ("2", 2),
        ("three", 3), ("3", 3),
        ("four", 4), ("4", 4),
        ("five", 5), ("5", 5),
        ("six", 6), ("6", 6),
        ("seven", 7), ("7", 7),
        ("eight", 8), ("8", 8),
        ("nine", 9), ("9", 9)];

    let min = numbers
        .iter()
        .map(|number| (line.find(number.0), number.1))
        .filter(|t| t.0.is_some())
        .min_by_key(|t| t.0)
        .unwrap()
        .1;

    let max = numbers
        .iter()
        .map(|number| (line.rfind(number.0), number.1))
        .filter(|t| t.0.is_some())
        .max_by_key(|t| t.0)
        .unwrap()
        .1;

    min * 10 + max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(44));
    }
}
