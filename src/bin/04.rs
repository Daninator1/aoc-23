use regex::Regex;
advent_of_code::solution!(4);

#[derive(Debug)]
struct Game {
    id: u32,
    winning_numbers: Vec<u32>,
    own_numbers: Vec<u32>,
    instances: u32,
}

impl Game {
    fn create(line: &str) -> Game {
        let whitespace_regex = Regex::new(r"\s+").unwrap();
        let normalized_line = whitespace_regex.replace_all(line, " ").into_owned();

        let game_regex = Regex::new(r"Card (?<game_id>\d+):(?<winning_numbers>[ \d]+)\|(?<own_numbers>[ \d]+)").unwrap();
        let Some((_, [game_id_str, winning_numbers_str, own_numbers_str])) = game_regex.captures(&normalized_line).map(|caps| caps.extract()) else { panic!("invalid line") };

        let game_id = game_id_str.trim().parse::<u32>().unwrap();
        let winning_numbers = winning_numbers_str.trim().split(' ').map(|x| x.parse::<u32>().unwrap()).collect();
        let own_numbers = own_numbers_str.trim().split(' ').map(|x| x.parse::<u32>().unwrap()).collect();
        Game { id: game_id, winning_numbers, own_numbers, instances: 1 }
    }

    fn increase_instances(&mut self) {
        self.instances += 1;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = input
        .lines()
        .map(Game::create);

    let sum = games
        .map(|game| {
            let matching_numbers: Vec<&u32> = game.winning_numbers.iter().filter(|winning_number| game.own_numbers.contains(winning_number)).collect();

            return match matching_numbers[..] {
                [] => 0,
                [_] => 1,
                _ => matching_numbers[1..].iter().fold(1, |acc, _| acc * 2)
            };
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut games: Vec<Game> = input
        .lines()
        .map(Game::create)
        .collect();

    for index in 0..games.len() {
        for _ in 0..games[index].instances {
            let matches = games[index].winning_numbers.iter().filter(|winning_number| games[index].own_numbers.contains(winning_number)).count();
            for m in 0..matches {
                let Some(other_game) = games.get_mut(index + 1 + m) else { break; };
                other_game.increase_instances();
            }
        }
    }

    let card_count = games.iter().map(|game| game.instances).sum();

    Some(card_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
