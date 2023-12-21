advent_of_code::solution!(15);

fn calc_hash(step: &str) -> usize {
    step
        .chars()
        .filter(|c| c != &'\n')
        .map(|c| c as usize)
        .fold(0, |acc, ascii| ((acc + ascii) * 17) % 256)
}

pub fn part_one(input: &str) -> Option<usize> {
    let result = input.split(',').map(calc_hash).sum();
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
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
