use std::{collections::HashMap, ops::Deref};

advent_of_code::solution!(7);

#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Card {
    _Placeholder, // This is here so nothing else is zero so I can set the jack to zero in part two
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn order(self) -> u8 {
        self as u8
    }

    fn order_part_two(self) -> u8 {
        match self {
            Card::Jack => 0,
            _ => self.order(),
        }
    }
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

impl HandKind {
    fn from(cards: &[Card], is_part_two: bool) -> Self {
        let mut map: HashMap<Card, usize> = cards.iter().fold(HashMap::new(), |mut map, curr| {
            *map.entry(*curr).or_insert(0) += 1;
            map
        });

        if is_part_two {
            if let Some(jacks) = map.remove(&Card::Jack) {
                let (card, _) = map
                    .iter()
                    .max_by(|a, b| a.1.cmp(b.1))
                    .unwrap_or((&Card::Ace, &0));
                *map.entry(*card).or_insert(0) += jacks;
            }
        }

        Self::match_entries(&map)
    }

    fn match_entries(map: &HashMap<Card, usize>) -> Self {
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

#[derive(Debug)]
struct Hand {
    kind: HandKind,
    card_order: Vec<u8>,
    bid: usize,
}

impl Hand {
    fn parse(line: &str, is_part_two: bool) -> Self {
        let (cards, bid) = line.split_once(' ').unwrap();
        let cards = cards.chars().map(Card::from).collect::<Vec<Card>>();
        let kind = HandKind::from(cards.deref(), is_part_two);
        let card_order = Hand::order_cards(&cards, is_part_two);

        Self {
            kind,
            card_order,
            bid: bid.parse().unwrap(),
        }
    }

    fn order_cards(cards: &[Card], is_part_two: bool) -> Vec<u8> {
        if is_part_two {
            cards.iter().map(|c| c.order_part_two()).collect::<Vec<_>>()
        } else {
            cards.iter().map(|c| c.order()).collect::<Vec<_>>()
        }
    }
}

fn solve(input: &str, is_part_two: bool) -> Option<usize> {
    let mut hands = input
        .trim()
        .lines()
        .map(|l| Hand::parse(l, is_part_two))
        .collect::<Vec<Hand>>();

    hands.sort_by(|a, b| {
        match a.kind.cmp(&b.kind) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        b.card_order.cmp(&a.card_order)
    });

    let result = hands
        .iter()
        .rev()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum();

    Some(result)
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, true)
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
