use std::{collections::HashMap, ops::Deref};

advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => unreachable!("This card shouldn't exist"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandKind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    DoublePair,
    Pair,
    HighCard,
}

impl From<&[Card]> for HandKind {
    fn from(cards: &[Card]) -> Self {
        let map = cards.iter().fold(HashMap::new(), |mut map, curr| {
            *map.entry(curr).or_insert(0) += 1;
            map
        });

        match map.len() {
            5 => Self::HighCard,
            4 => Self::Pair,
            3 => {
                if map.iter().any(|(_card, &count)| count == 3) {
                    Self::ThreeOfAKind
                } else {
                    Self::DoublePair
                }
            }
            2 => {
                if map.iter().any(|(_card, &count)| count == 4) {
                    Self::FourOfAKind
                } else {
                    Self::FullHouse
                }
            }
            1 => Self::FiveOfAKind,
            _ => unreachable!("There are no other possible types of hand"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Hand {
    kind: HandKind,
    cards: Vec<Card>,
    bid: usize,
}

impl Hand {
    fn parse(line: &str) -> Self {
        let (cards, bid) = line.split_once(' ').unwrap();
        let cards = cards.chars().map(Card::from).collect::<Vec<Card>>();
        let kind = HandKind::from(cards.deref());

        Self {
            kind,
            cards,
            bid: bid.parse().unwrap(),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut hands = input.trim().lines().map(Hand::parse).collect::<Vec<Hand>>();

    hands.sort();
    // dbg!(&hands);

    let result = hands
        .iter()
        .rev()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
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
        assert_eq!(result, None);
    }
}
