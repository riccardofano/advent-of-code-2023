advent_of_code::solution!(8);

use std::{
    cmp::{max, min},
    collections::HashMap,
};

fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

struct Map<'a> {
    directions: Vec<usize>,
    graph: HashMap<&'a str, [&'a str; 2]>,
}

impl<'a> Map<'a> {
    fn parse(input: &'a str) -> Self {
        let (directions, nodes) = input.trim().split_once("\n\n").unwrap();
        let directions = directions
            .chars()
            .map(|c| match c {
                'L' => 0,
                'R' => 1,
                _ => unreachable!("There should only be L and R"),
            })
            .collect::<Vec<usize>>();

        let graph = nodes
            .lines()
            .map(|l| {
                let (node, connections) = l.split_once(" = ").unwrap();
                let (left, right) = connections
                    .strip_prefix('(')
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap()
                    .split_once(", ")
                    .unwrap();

                (node, [left, right])
            })
            .collect::<HashMap<&str, [&str; 2]>>();

        Self { directions, graph }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map::parse(input);

    let mut steps = 0;
    let num_directions = map.directions.len();

    let mut current_node = "AAA";
    while current_node != "ZZZ" {
        let next_direction = map.directions[steps % num_directions];
        current_node = map.graph.get(current_node).unwrap()[next_direction];
        steps += 1;
    }

    Some(steps)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Map::parse(input);
    let num_directions = map.directions.len();

    let starting_nodes = map
        .graph
        .keys()
        .filter(|n| n.ends_with('A'))
        .collect::<Vec<_>>();

    let mut steps_taken = vec![0; starting_nodes.len()];
    for (i, &&node) in starting_nodes.iter().enumerate() {
        let mut current = node;

        while current.as_bytes().last() != Some(&b'Z') {
            let next_direction = map.directions[steps_taken[i] % num_directions];
            current = map.graph.get(current).unwrap()[next_direction];
            steps_taken[i] += 1;
        }
    }

    let mut least_common_multiple = steps_taken[0];
    for finish in steps_taken.iter().skip(1) {
        least_common_multiple = lcm(least_common_multiple, *finish);
    }

    Some(least_common_multiple)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));

        let input2 = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
        assert_eq!(part_one(input2), Some(6))
    }

    #[test]
    fn test_part_two() {
        let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
        let result = part_two(input);
        assert_eq!(result, Some(6));
    }
}
