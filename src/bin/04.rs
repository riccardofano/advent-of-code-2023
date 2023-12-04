use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let result = input
        .trim()
        .lines()
        .map(|l| {
            let (_card_info, numbers) = l.split_once(": ").unwrap();

            let (winning, have) = numbers.split_once(" | ").unwrap();
            let winning_set = winning
                .split_whitespace()
                .map(|n| n.parse())
                .collect::<Result<HashSet<usize>, _>>()
                .unwrap();

            let count = have
                .split_whitespace()
                .filter_map(|n| n.parse::<usize>().ok())
                .filter_map(|n| winning_set.get(&n))
                .count();

            // I'm sure there is a better way but it's coming to me right now
            let mut sum = 0;
            for i in 0..count {
                if i == 0 {
                    sum += 1;
                } else {
                    sum *= 2;
                }
            }
            sum
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let winnings_per_card = input
        .trim()
        .lines()
        .map(|l| {
            let (_card_info, numbers) = l.split_once(": ").unwrap();

            let (winning, have) = numbers.split_once(" | ").unwrap();
            let winning_set = winning
                .split_whitespace()
                .map(|n| n.parse())
                .collect::<Result<HashSet<usize>, _>>()
                .unwrap();

            have.split_whitespace()
                .filter_map(|n| n.parse::<usize>().ok())
                .filter_map(|n| winning_set.get(&n))
                .count()
        })
        .collect::<Vec<usize>>();

    let mut card_instances = vec![1; winnings_per_card.len()];
    for (index, &winning_cards) in winnings_per_card.iter().enumerate() {
        for i in 0..winning_cards {
            card_instances[index + i + 1] += card_instances[index];
        }
    }

    Some(card_instances.iter().sum())
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
