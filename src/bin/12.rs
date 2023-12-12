use std::collections::HashMap;

advent_of_code::solution!(12);

fn parse_line(line: &str) -> (&str, Vec<usize>) {
    let (pattern, gear_lengths) = line.split_once(' ').unwrap();
    let gear_lengths = gear_lengths
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    (pattern, gear_lengths)
}

fn recurse(pattern: &str, gear_lengths: &[usize], cache: &mut HashMap<String, usize>) -> usize {
    let key = format!(
        "{pattern} {}",
        gear_lengths
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    if !cache.contains_key(&key) {
        let res = process(pattern, gear_lengths, cache);
        cache.insert(key.clone(), res);
    }

    *cache.get(&key).unwrap()
}

fn process(pattern: &str, gear_lengths: &[usize], cache: &mut HashMap<String, usize>) -> usize {
    let mut iter = pattern.chars();

    let result = match iter.next() {
        None => gear_lengths.is_empty() as usize,
        Some('.') => recurse(iter.as_str(), gear_lengths, cache),
        Some('?') => {
            let rest = iter.as_str();
            let try_dot = format!(".{rest}");
            let try_hash = format!("#{rest}");

            recurse(&try_dot, gear_lengths, cache) + recurse(&try_hash, gear_lengths, cache)
        }
        Some('#') => count_damaged_gears(pattern, gear_lengths, cache),
        Some(c) => panic!("Unknown character, {c:?}"),
    };

    result
}

fn count_damaged_gears(
    pattern: &str,
    gear_lengths: &[usize],
    cache: &mut HashMap<String, usize>,
) -> usize {
    if gear_lengths.is_empty() {
        return 0;
    }

    let correct_length = gear_lengths[0];
    let gear_lengths = &gear_lengths[1..];

    let possibly_damaged_section = pattern
        .chars()
        .take_while(|&part| part == '#' || part == '?')
        .count();

    if possibly_damaged_section < correct_length {
        return 0;
    }
    if pattern.len() == correct_length {
        return recurse("", gear_lengths, cache);
    }
    if pattern.chars().nth(correct_length) == Some('#') {
        return 0;
    }

    let rest_of_pattern: String = pattern.chars().skip(correct_length + 1).collect();
    recurse(&rest_of_pattern, gear_lengths, cache)
}

pub fn part_one(input: &str) -> Option<usize> {
    let result = input
        .trim()
        .lines()
        .map(parse_line)
        .map(|(pattern, gear_lengths)| {
            let mut cache = HashMap::new();
            recurse(pattern, &gear_lengths, &mut cache)
        })
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
