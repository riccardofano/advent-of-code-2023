advent_of_code::solution!(5);

use std::{collections::HashMap, ops::Range};

fn insert_ranges(data: &str, map: &mut HashMap<Range<usize>, Range<usize>>) {
    data.lines().for_each(|l| {
        let [dest_start, source_start, range_len]: [usize; 3] = l
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        map.insert(
            source_start..source_start + range_len,
            dest_start..dest_start + range_len,
        );
    })
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
    let parts = input.trim().split("\n\n").collect::<Vec<_>>();

    let seeds = parts[0]
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();

    let parts = parts
        .iter()
        .skip(1)
        .map(|p| p.lines().skip(1).collect::<Vec<_>>().join("\n"))
        .collect::<Vec<_>>();

    let mut seed_to_soil = HashMap::new();
    insert_ranges(&parts[0], &mut seed_to_soil);
    let mut soil_to_fertilizer = HashMap::new();
    insert_ranges(&parts[1], &mut soil_to_fertilizer);
    let mut fertilizer_to_water = HashMap::new();
    insert_ranges(&parts[2], &mut fertilizer_to_water);
    let mut water_to_light = HashMap::new();
    insert_ranges(&parts[3], &mut water_to_light);
    let mut light_to_temp = HashMap::new();
    insert_ranges(&parts[4], &mut light_to_temp);
    let mut temp_to_humidity = HashMap::new();
    insert_ranges(&parts[5], &mut temp_to_humidity);
    let mut humidity_to_location = HashMap::new();
    insert_ranges(&parts[6], &mut humidity_to_location);

    let lowest_location = seeds
        .iter()
        .map(|&s| number_in_available_ranges(s, &seed_to_soil).unwrap_or(s))
        .map(|s| number_in_available_ranges(s, &soil_to_fertilizer).unwrap_or(s))
        .map(|s| number_in_available_ranges(s, &fertilizer_to_water).unwrap_or(s))
        .map(|s| number_in_available_ranges(s, &water_to_light).unwrap_or(s))
        .map(|s| number_in_available_ranges(s, &light_to_temp).unwrap_or(s))
        .map(|s| number_in_available_ranges(s, &temp_to_humidity).unwrap_or(s))
        .map(|s| number_in_available_ranges(s, &humidity_to_location).unwrap_or(s))
        .min()
        .unwrap();

    Some(lowest_location)
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
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
