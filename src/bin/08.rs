advent_of_code::solution!(8);

use std::cmp::{max, min};
use std::collections::HashMap;

use winnow::ascii::{alphanumeric1, line_ending};
use winnow::combinator::{delimited, iterator, opt, repeat, separated_pair, terminated};
use winnow::stream::AsChar;
use winnow::token::{any, take_till};
use winnow::{PResult, Parser};

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

type NodeMap<'a> = HashMap<&'a str, [&'a str; 2]>;
type NodeElem<'a> = (&'a str, [&'a str; 2]);

struct Map<'a> {
    directions: Vec<usize>,
    graph: NodeMap<'a>,
}

fn parse_sections<'a>(input: &mut &'a str) -> PResult<(Vec<usize>, NodeMap<'a>)> {
    separated_pair(parse_directions, "\n\n", parse_nodes).parse_next(input)
}

fn parse_direction(input: &mut &str) -> PResult<usize> {
    any.map(|c| match c {
        'L' => 0,
        'R' => 1,
        _ => panic!("Expected to only get L or R, got: {c:?}"),
    })
    .parse_next(input)
}

fn parse_directions(input: &mut &str) -> PResult<Vec<usize>> {
    let mut directions = take_till(1.., AsChar::is_newline).parse_next(input)?;
    repeat(1.., parse_direction).parse_next(&mut directions)
}

fn parse_left_right<'a>(input: &mut &'a str) -> PResult<[&'a str; 2]> {
    delimited('(', separated_pair(alphanumeric1, ", ", alphanumeric1), ')')
        .map(|(l, r)| [l, r])
        .parse_next(input)
}

fn parse_node<'a>(input: &mut &'a str) -> PResult<NodeElem<'a>> {
    terminated(
        separated_pair(alphanumeric1, " = ", parse_left_right),
        opt(line_ending),
    )
    .parse_next(input)
}

fn parse_nodes<'a>(input: &mut &'a str) -> PResult<NodeMap<'a>> {
    let mut it = iterator(*input, parse_node);
    let parsed = it.collect::<HashMap<_, _>>();

    it.finish()?;

    Ok(parsed)
}

impl<'a> Map<'a> {
    fn parse(mut input: &'a str) -> Self {
        let (directions, graph) = parse_sections.parse_next(&mut input).unwrap();
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
    fn test_parsing_direction() {
        assert_eq!(parse_direction(&mut "L"), Ok(0))
    }

    #[test]
    fn test_parsing_directions() {
        assert_eq!(parse_directions(&mut "LR\n"), Ok(vec![0, 1]))
    }

    #[test]
    fn test_parsing_left_right() {
        assert_eq!(parse_left_right(&mut "(AAA, ZZZ)"), Ok(["AAA", "ZZZ"]))
    }

    #[test]
    fn test_parsing_node() {
        assert_eq!(
            parse_node(&mut "CCC = (AAA, ZZZ)"),
            Ok(("CCC", ["AAA", "ZZZ"]))
        )
    }

    #[test]
    fn test_parsing_nodes() {
        let mut input = "AAA = (BBB, CCC)\nBBB = (DDD, EEE)";
        let expected = HashMap::from([("AAA", ["BBB", "CCC"]), ("BBB", ["DDD", "EEE"])]);

        assert_eq!(parse_nodes(&mut input), Ok(expected))
    }

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
