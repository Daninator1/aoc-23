use std::collections::hash_map::Keys;
use std::collections::HashMap;
advent_of_code::solution!(2);

#[derive(Eq, PartialEq, Hash)]
enum ColorDraw {
    Red { amount: u32 },
    Blue { amount: u32 },
    Green { amount: u32 },
}

struct Game {
    id: u32,
    draws: HashMap<usize, Vec<ColorDraw>>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = input.lines().map(|line| {
        let mut parts = line.split(':');

        let game_id = parts
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let draw_parts = parts.last().unwrap().trim().split(';');

        let mut all_draws: HashMap<usize, Vec<ColorDraw>> = HashMap::new();

        draw_parts.enumerate().for_each(|(index, draw_part)| {
            let draws: Vec<ColorDraw> = draw_part.trim().split(',').map(|draw| {
                let mut draw_pair = draw.trim().split(' ');
                match draw_pair.clone().last() {
                    Some("red") => ColorDraw::Red { amount: draw_pair.next().unwrap().parse::<u32>().unwrap() },
                    Some("blue") => ColorDraw::Blue { amount: draw_pair.next().unwrap().parse::<u32>().unwrap() },
                    Some("green") => ColorDraw::Green { amount: draw_pair.next().unwrap().parse::<u32>().unwrap() },
                    _ => panic!("invalid color")
                }
            }).collect();

            all_draws.insert(index, draws);
        });

        Game {
            id: game_id,
            draws: all_draws,
        }
    });

    let valid_games = games.filter(|game| game.draws.iter().all(|(_, draws)| draws.iter().all(|draw| match draw {
        ColorDraw::Red { amount } => amount <= &12,
        ColorDraw::Blue { amount } => amount <= &14,
        ColorDraw::Green { amount } => amount <= &13,
    })));

    Some(valid_games.map(|game| game.id).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = input.lines().map(|line| {
        let mut parts = line.split(':');

        let game_id = parts
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let draw_parts = parts.last().unwrap().trim().split(';');

        let mut all_draws: HashMap<usize, Vec<ColorDraw>> = HashMap::new();

        draw_parts.enumerate().for_each(|(index, draw_part)| {
            let draws: Vec<ColorDraw> = draw_part.trim().split(',').map(|draw| {
                let mut draw_pair = draw.trim().split(' ');
                match draw_pair.clone().last() {
                    Some("red") => ColorDraw::Red { amount: draw_pair.next().unwrap().parse::<u32>().unwrap() },
                    Some("blue") => ColorDraw::Blue { amount: draw_pair.next().unwrap().parse::<u32>().unwrap() },
                    Some("green") => ColorDraw::Green { amount: draw_pair.next().unwrap().parse::<u32>().unwrap() },
                    _ => panic!("invalid color")
                }
            }).collect();

            all_draws.insert(index, draws);
        });

        Game {
            id: game_id,
            draws: all_draws,
        }
    });

    Some(games.map(|game| {
        let draws = game.draws.values().flatten();

        let mut max_red: u32 = 0;
        let mut max_blue: u32 = 0;
        let mut max_green: u32 = 0;

        draws.for_each(|draw| {
            match draw {
                ColorDraw::Red { amount } => max_red = if amount > &max_red { *amount } else { max_red },
                ColorDraw::Blue { amount } => max_blue = if amount > &max_blue { *amount } else { max_blue },
                ColorDraw::Green { amount } => max_green = if amount > &max_green { *amount } else { max_green },
            }
        });

        max_red * max_blue * max_green
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
