advent_of_code::solution!(9);

fn overlapping_chunks<T: Clone>(vec: &Vec<T>, chunk_size: usize) -> impl Iterator<Item=&[T]> {
    (0..vec.len() - chunk_size + 1).map(move |i| &vec[i..i + chunk_size])
}

fn check(numbers: Vec<i32>) -> i32 {
    let mut history: Vec<Vec<i32>> = vec![numbers];

    loop {
        let new_numbers: Vec<i32> = overlapping_chunks(history.last().unwrap(), 2).map(|chunk| {
            match chunk {
                [a, b] => b - a,
                _ => 0
            }
        }).collect();

        if new_numbers.iter().all(|n| n == &0) { break; }

        history.push(new_numbers);
    }

    let result = history.iter().rfold(0, |acc, x| {
        let new_number = x.last().unwrap() + acc;
        new_number
    });

    result
}

fn check_2(numbers: Vec<i32>) -> i32 {
    let mut history: Vec<Vec<i32>> = vec![numbers];

    loop {
        let new_numbers: Vec<i32> = overlapping_chunks(history.last().unwrap(), 2).map(|chunk| {
            match chunk {
                [a, b] => b - a,
                _ => 0
            }
        }).collect();

        if new_numbers.iter().all(|n| n == &0) { break; }

        history.push(new_numbers);
    }

    let result = history.iter().rfold(0, |acc, x| {
        let new_number = x.iter().next().unwrap() - acc;
        new_number
    });

    result
}

pub fn part_one(input: &str) -> Option<i32> {
    let numbers_list: Vec<Vec<i32>> = input.lines().map(|line| line.split(' ').map(|n| n.parse::<i32>().unwrap()).collect()).collect();

    let results = numbers_list.iter().map(|numbers| check(numbers.to_vec()));

    Some(results.sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let numbers_list: Vec<Vec<i32>> = input.lines().map(|line| line.split(' ').map(|n| n.parse::<i32>().unwrap()).collect()).collect();

    let results = numbers_list.iter().map(|numbers| check_2(numbers.to_vec()));

    Some(results.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
