advent_of_code::solution!(5);

use std::{collections::HashMap, ops::Range};

fn map_range(data: &str) -> Vec<(Range<usize>, Range<usize>)> {
    data.lines()
        .map(|l| {
            let [dest_start, source_start, range_len]: [usize; 3] = l
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            (
                source_start..source_start + range_len,
                dest_start..dest_start + range_len,
            )
        })
        .collect()
}

fn number_in_available_ranges(
    number: usize,
    map: &HashMap<Range<usize>, Range<usize>>,
) -> Option<usize> {
    for (source, dest) in map.iter() {
        if source.contains(&number) {
            let difference = source.start.abs_diff(dest.start);
            if source.start < dest.start {
                return Some(number + difference);
            } else {
                return Some(number - difference);
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut parts = input.trim().split("\n\n");

    let seeds = parts
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();

    let maps = parts
        .map(|p| {
            p.lines()
                .skip(1)
                .flat_map(map_range)
                .collect::<HashMap<Range<usize>, Range<usize>>>()
        })
        .collect::<Vec<_>>();

    let result = seeds
        .iter()
        .map(|&s| {
            maps.iter().fold(s, |res, map| {
                number_in_available_ranges(res, map).unwrap_or(res)
            })
        })
        .min();

    result
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut parts = input.trim().split("\n\n");

    let seeds = parts
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut seed_ranges = Vec::new();
    let mut i = 0;
    while i < seeds.len() {
        seed_ranges.push(seeds[i]..seeds[i] + seeds[i + 1]);
        i += 2;
    }

    let mut seeds = Vec::new();
    for range in seed_ranges.iter() {
        for i in range.start..range.end {
            seeds.push(i);
        }
    }

    let maps = parts
        .map(|p| {
            p.lines()
                .skip(1)
                .flat_map(map_range)
                .collect::<HashMap<Range<usize>, Range<usize>>>()
        })
        .collect::<Vec<_>>();

    let result = seeds
        .iter()
        .map(|&s| {
            maps.iter().fold(s, |res, map| {
                number_in_available_ranges(res, map).unwrap_or(res)
            })
        })
        .min();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
