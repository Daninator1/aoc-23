use std::cmp::Ordering;
use std::str::FromStr;
use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
advent_of_code::solution!(7);

#[derive(Debug)]
struct Game {
    cards: Vec<CardLabel>,
    hand_type: HandType,
    bid: usize,
}

impl Eq for Game {}

impl PartialEq<Self> for Game {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for (own_card, other_card) in self.cards.iter().zip(&other.cards) {
                    match own_card.cmp(other_card) {
                        Ordering::Less => { return Ordering::Less; }
                        Ordering::Equal => {}
                        Ordering::Greater => { return Ordering::Greater; }
                    }
                }

                Ordering::Equal
            }
            Ordering::Greater => Ordering::Greater,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct ParseGameError {
    message: String,
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 2 { return Err(ParseGameError { message: "invalid length of string parts".into() }); };
        let (cards_part, bid_part) = (parts[0], parts[1]);

        let cards: Vec<CardLabel> = match cards_part.chars().map(CardLabel::try_from).collect() {
            Ok(cards) => cards,
            Err(e) => return Err(ParseGameError { message: e.message }),
        };

        let hand_type = match HandType::try_from(&cards) {
            Ok(hand_type) => hand_type,
            Err(e) => return Err(ParseGameError { message: e.message }),
        };

        let bid = match bid_part.parse::<usize>() {
            Ok(bid) => bid,
            Err(_) => return Err(ParseGameError { message: "failed to parse bid".into() }),
        };

        Ok(Game {
            cards,
            hand_type,
            bid,
        })
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Hash, EnumIter, Copy, Clone)]
enum CardLabel {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl CardLabel {
    fn get_combinations(cards: &[CardLabel]) -> Vec<Vec<CardLabel>> {
        let mut result: Vec<Vec<CardLabel>> = Vec::new();

        match cards[..] {
            [CardLabel::Joker, b, c, d, e] => {
                for card_label in CardLabel::iter().filter(|cl| cl != &CardLabel::Joker) {
                    result.append(&mut CardLabel::get_combinations(&[card_label, b, c, d, e]).into_iter().collect::<Vec<Vec<CardLabel>>>());
                }
            }
            [a, CardLabel::Joker, c, d, e] => {
                for card_label in CardLabel::iter().filter(|cl| cl != &CardLabel::Joker) {
                    result.append(&mut CardLabel::get_combinations(&[a, card_label, c, d, e]).into_iter().collect::<Vec<Vec<CardLabel>>>());
                }
            }
            [a, b, CardLabel::Joker, d, e] => {
                for card_label in CardLabel::iter().filter(|cl| cl != &CardLabel::Joker) {
                    result.append(&mut CardLabel::get_combinations(&[a, b, card_label, d, e]).into_iter().collect::<Vec<Vec<CardLabel>>>());
                }
            }
            [a, b, c, CardLabel::Joker, e] => {
                for card_label in CardLabel::iter().filter(|cl| cl != &CardLabel::Joker) {
                    result.append(&mut CardLabel::get_combinations(&[a, b, c, card_label, e]).into_iter().collect::<Vec<Vec<CardLabel>>>());
                }
            }
            [a, b, c, d, CardLabel::Joker] => {
                for card_label in CardLabel::iter().filter(|cl| cl != &CardLabel::Joker) {
                    result.append(&mut CardLabel::get_combinations(&[a, b, c, d, card_label]).into_iter().collect::<Vec<Vec<CardLabel>>>());
                }
            }
            _ => { result.push(cards.to_vec()) }
        }

        result
    }
}

#[derive(Eq, PartialEq, Debug)]
struct ParseCardLabelError {
    message: String,
}

impl TryFrom<char> for CardLabel {
    type Error = ParseCardLabelError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(CardLabel::Joker), // modified the char to be compatible with day 2
            '2' => Ok(CardLabel::Two),
            '3' => Ok(CardLabel::Three),
            '4' => Ok(CardLabel::Four),
            '5' => Ok(CardLabel::Five),
            '6' => Ok(CardLabel::Six),
            '7' => Ok(CardLabel::Seven),
            '8' => Ok(CardLabel::Eight),
            '9' => Ok(CardLabel::Nine),
            'T' => Ok(CardLabel::T),
            'J' => Ok(CardLabel::J),
            'Q' => Ok(CardLabel::Q),
            'K' => Ok(CardLabel::K),
            'A' => Ok(CardLabel::A),
            x => Err(ParseCardLabelError { message: format!("invalid card label {0}", x) })
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn get_hand_type(cards: &[CardLabel]) -> Option<HandType> {
        if cards.iter().all_equal() {
            return Some(HandType::FiveOfAKind);
        }

        if cards.iter().all_unique() {
            return Some(HandType::HighCard);
        }

        let groups: Vec<(&CardLabel, usize)> = cards.iter()
            .sorted()
            .group_by(|card| *card)
            .into_iter()
            .map(|(card, group)| (card, group.count()))
            .collect();

        if groups.iter().any(|(_, count)| count == &4) {
            return Some(HandType::FourOfAKind);
        }

        if groups.iter().any(|(_, count)| count == &3) &&
            groups.iter().any(|(_, count)| count == &2) {
            return Some(HandType::FullHouse);
        }

        if groups.iter().any(|(_, count)| count == &3) &&
            !groups.iter().any(|(_, count)| count == &2) {
            return Some(HandType::ThreeOfAKind);
        }

        if groups.iter().filter(|(_, count)| count == &2).count() == 2 {
            return Some(HandType::TwoPair);
        }

        if groups.iter().filter(|(_, count)| count == &2).count() == 1 {
            return Some(HandType::OnePair);
        }

        None
    }
}

#[derive(Eq, PartialEq, Debug)]
struct ParseHandTypeError {
    message: String,
}

impl TryFrom<&Vec<CardLabel>> for HandType {
    type Error = ParseHandTypeError;

    fn try_from(cards: &Vec<CardLabel>) -> Result<Self, Self::Error> {
        let possible_hand_types: Vec<HandType> = CardLabel::get_combinations(cards).iter()
            .filter_map(|combination| HandType::get_hand_type(combination))
            .collect();

        let best_hand_type = possible_hand_types.iter().max();

        match best_hand_type {
            None => Err(ParseHandTypeError { message: "no hand type found".into() }),
            Some(hand_type) => Ok(*hand_type),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let games: Vec<Game> = input
        .lines()
        .map(|line| line.parse::<Game>())
        .collect::<Result<Vec<Game>, ParseGameError>>()
        .expect("game parsing error");

    let result: usize = games.iter()
        .sorted().enumerate()
        .map(|(index, game)| game.bid * (index + 1))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let games: Vec<Game> = input
        .lines()
        .map(|line| line.replace('J', "X"))
        .map(|line| line.parse::<Game>())
        .collect::<Result<Vec<Game>, ParseGameError>>()
        .expect("game parsing error");

    let result: usize = games.iter()
        .sorted().enumerate()
        .map(|(index, game)| game.bid * (index + 1))
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
