advent_of_code::solution!(3);

#[derive(Debug)]
struct Number {
    value_str: String,
    position: Position,
}

impl Number {
    fn is_part(&self, symbols: &[Symbol]) -> bool {
        for index in (0..self.value_str.len()).collect::<Vec<usize>>() {
            let possible_collisions: Vec<Position> = vec![
                Position { x: self.position.x + index as i32 - 1, y: self.position.y },
                Position { x: self.position.x + index as i32 - 1, y: self.position.y - 1 },
                Position { x: self.position.x + index as i32, y: self.position.y - 1 },
                Position { x: self.position.x + index as i32 + 1, y: self.position.y - 1 },
                Position { x: self.position.x + index as i32 + 1, y: self.position.y },
                Position { x: self.position.x + index as i32 + 1, y: self.position.y + 1 },
                Position { x: self.position.x + index as i32, y: self.position.y + 1 },
                Position { x: self.position.x + index as i32 - 1, y: self.position.y + 1 },
            ];

            let collision = symbols.iter().any(|symbol| possible_collisions.contains(&symbol.position));

            if collision {
                return true;
            }
        }

        false
    }

    fn to_number(&self) -> u32 {
        self.value_str.parse::<u32>().unwrap()
    }

    fn get_positions(&self) -> Vec<Position> {
        let mut positions: Vec<Position> = vec![self.position];

        for index in (1..self.value_str.len()).collect::<Vec<usize>>() {
            positions.push(Position { x: self.position.x + index as i32, y: self.position.y })
        }

        positions
    }
}

#[derive(Debug)]
struct Symbol {
    is_gear: bool,
    position: Position,
}

impl Symbol {
    fn calculate_gear_ratio(&self, numbers: &[&Number]) -> Option<u32> {
        let possible_collisions: Vec<Position> = vec![
            Position { x: self.position.x - 1, y: self.position.y },
            Position { x: self.position.x - 1, y: self.position.y - 1 },
            Position { x: self.position.x, y: self.position.y - 1 },
            Position { x: self.position.x + 1, y: self.position.y - 1 },
            Position { x: self.position.x + 1, y: self.position.y },
            Position { x: self.position.x + 1, y: self.position.y + 1 },
            Position { x: self.position.x, y: self.position.y + 1 },
            Position { x: self.position.x - 1, y: self.position.y + 1 },
        ];

        let colliding_numbers = numbers.iter().filter(|number| {
            let number_positions = number.get_positions();
            number_positions.iter().any(|position| possible_collisions.contains(position))
        });

        if colliding_numbers.clone().count() == 2 {
            Some(colliding_numbers.map(|number| number.to_number()).product())
        } else {
            None
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for (line_index, line) in input.lines().enumerate() {
        let mut curr_number_str: String = String::new();
        let mut curr_number_pos: Position = Position { x: 0, y: line_index as i32 };

        for (char_index, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                curr_number_str.push(char);
            } else {
                if char != '.' {
                    symbols.push(Symbol { is_gear: char == '*', position: Position { x: char_index as i32, y: line_index as i32 } });
                }

                if !curr_number_str.is_empty() {
                    numbers.push(Number { value_str: curr_number_str.clone(), position: curr_number_pos });
                    curr_number_str = String::new();
                }

                curr_number_pos = Position { x: char_index as i32 + 1, y: line_index as i32 };
            }
        }

        if !curr_number_str.is_empty() {
            numbers.push(Number { value_str: curr_number_str.clone(), position: curr_number_pos });
        }
    }

    let part_numbers = numbers.iter().filter(|number| number.is_part(&symbols));

    Some(part_numbers.map(|part_number| part_number.to_number()).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for (line_index, line) in input.lines().enumerate() {
        let mut curr_number_str: String = String::new();
        let mut curr_number_pos: Position = Position { x: 0, y: line_index as i32 };

        for (char_index, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                curr_number_str.push(char);
            } else {
                if char != '.' {
                    symbols.push(Symbol { is_gear: char == '*', position: Position { x: char_index as i32, y: line_index as i32 } });
                }

                if !curr_number_str.is_empty() {
                    numbers.push(Number { value_str: curr_number_str.clone(), position: curr_number_pos });
                    curr_number_str = String::new();
                }

                curr_number_pos = Position { x: char_index as i32 + 1, y: line_index as i32 };
            }
        }

        if !curr_number_str.is_empty() {
            numbers.push(Number { value_str: curr_number_str.clone(), position: curr_number_pos });
        }
    }

    let part_numbers: Vec<&Number> = numbers.iter().filter(|number| number.is_part(&symbols)).collect();
    let gears = symbols.iter().filter(|symbol| symbol.is_gear);

    Some(gears.filter_map(|gear| gear.calculate_gear_ratio(&part_numbers)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
