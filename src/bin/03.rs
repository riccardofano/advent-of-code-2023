advent_of_code::solution!(3);

use regex::Regex;
use std::collections::{HashMap, HashSet};

type Point = (usize, usize);

/// Doesn't matter if they're outside the bounds
fn neighbors((x, y): Point) -> Vec<Point> {
    vec![
        (x.saturating_sub(1), y.saturating_sub(1)),
        (x.saturating_sub(1), y),
        (x.saturating_sub(1), y + 1),
        (x, y.saturating_sub(1)),
        (x, y + 1),
        (x + 1, y.saturating_sub(1)),
        (x + 1, y),
        (x + 1, y + 1),
    ]
}

fn part_one(input: &str) -> Option<usize> {
    let lines = input.trim().lines().collect::<Vec<_>>();

    let re_number = Regex::new(r"\d+").unwrap();

    let mut numbers: Vec<(&str, Point)> = Vec::new();
    let mut symbols: HashSet<Point> = HashSet::new();

    for (row, line) in lines.iter().enumerate() {
        for found in re_number.find_iter(line) {
            let number_str = found.as_str();
            numbers.push((number_str, (found.start(), row)))
        }

        for (col, char) in line.chars().enumerate() {
            if char != '.' && !char.is_ascii_digit() {
                symbols.insert((col, row));
            }
        }
    }

    let mut touching: Vec<usize> = Vec::new();
    for (number, (start_x, y)) in numbers {
        let end_x = start_x + number.len();

        'number_loop: for x in start_x..end_x {
            for neighbor in neighbors((x, y)) {
                let symbol = symbols.get(&neighbor);
                if symbol.is_some() {
                    touching.push(number.parse().unwrap());
                    break 'number_loop;
                }
            }
        }
    }

    Some(touching.into_iter().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines = input.trim().lines().collect::<Vec<_>>();

    let re_number = Regex::new(r"\d+").unwrap();

    let mut numbers: Vec<(&str, Point)> = Vec::new();
    let mut symbols: HashSet<Point> = HashSet::new();

    for (row, line) in lines.iter().enumerate() {
        for found in re_number.find_iter(line) {
            let number_str = found.as_str();
            numbers.push((number_str, (found.start(), row)))
        }

        for (col, char) in line.chars().enumerate() {
            if char == '*' {
                symbols.insert((col, row));
            }
        }
    }

    let mut touching: HashMap<Point, Vec<usize>> = HashMap::new();
    for (number, (start_x, y)) in numbers {
        let end_x = start_x + number.len();

        'number_loop: for x in start_x..end_x {
            for neighbor in neighbors((x, y)) {
                if let Some(symbol) = symbols.get(&neighbor) {
                    let ratios = touching.entry(*symbol).or_default();
                    ratios.push(number.parse().unwrap());

                    break 'number_loop;
                }
            }
        }
    }

    Some(
        touching
            .into_iter()
            .filter(|(_, vec)| vec.len() == 2)
            .map(|(_, vec)| vec[0] * vec[1])
            .sum(),
    )
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
