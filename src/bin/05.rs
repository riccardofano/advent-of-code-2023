advent_of_code::solution!(5);

use std::{
    collections::{BTreeSet, HashMap},
    ops::Range,
};

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

fn convert_range(
    range: &Range<usize>,
    map: &HashMap<Range<usize>, Range<usize>>,
) -> Vec<Range<usize>> {
    let mut slices = BTreeSet::new();

    for source in map.keys() {
        if range.end < source.start || range.start > source.end {
            continue;
        }

        if source.start > range.start {
            slices.insert(range.start);
        }
        if source.end < range.end {
            slices.insert(source.end);
        }
    }

    slices.insert(range.end);

    let mut output = Vec::new();
    let mut current = range.start;

    for position in slices {
        let length = position - current;
        let converted_start = number_in_available_ranges(current, map).unwrap_or(current);
        output.push(converted_start..converted_start + length);
        current = position;
    }

    output
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

    let seed_line = parts.next().expect("No seed section");
    let (_, seeds) = seed_line.split_once(": ").expect("No seed header");
    let seeds = seeds
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<usize>, _>>()
        .expect("Not all seeds were numbers");

    let seed_ranges = seeds
        .chunks(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect::<Vec<Range<usize>>>();

    let maps = parts
        .map(|p| {
            p.lines()
                .skip(1)
                .flat_map(map_range)
                .collect::<HashMap<Range<usize>, Range<usize>>>()
        })
        .collect::<Vec<_>>();

    let mut valid_ranges = seed_ranges;
    let mut transformed_ranges = Vec::new();

    for map in maps {
        for range in valid_ranges {
            transformed_ranges.extend(convert_range(&range, &map))
        }
        valid_ranges = transformed_ranges;
        transformed_ranges = Vec::new();
    }

    valid_ranges.iter().map(|range| range.start).min()
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
