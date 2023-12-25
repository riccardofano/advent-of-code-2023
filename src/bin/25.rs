use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(25);

type Graph<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn parse(input: &str) -> Graph {
    let mut graph: Graph = HashMap::new();

    for line in input.lines() {
        let (a, others) = line.split_once(": ").unwrap();

        for b in others.split_whitespace() {
            graph.entry(a).or_default().insert(b);
            graph.entry(b).or_default().insert(a);
        }
    }

    graph
}

fn find_most_common_connection<'a>(graph: &Graph<'a>) -> [&'a str; 2] {
    let mut connections: HashMap<[&str; 2], usize> = HashMap::new();
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    for vertex in graph.keys() {
        queue.clear();
        queue.push_back(vertex);

        seen.clear();
        seen.insert(vertex);

        while let Some(&v) = queue.pop_front() {
            for connection in graph[v].iter() {
                if seen.contains(connection) {
                    continue;
                }
                seen.insert(connection);

                queue.push_back(connection);
                let mut connection_pair = [v, connection];
                connection_pair.sort();

                *connections.entry(connection_pair).or_default() += 1;
            }
        }
    }

    connections
        .into_iter()
        .max_by_key(|(_pair, passed_over)| *passed_over)
        .unwrap()
        .0
}

fn calculate_graph_size(graph: &Graph<'_>) -> usize {
    let start = graph.keys().next().unwrap();

    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([start]);

    while let Some(vertex) = queue.pop_front() {
        if visited.contains(&vertex) {
            continue;
        }
        visited.insert(vertex);

        for connection in graph[vertex].iter() {
            if !visited.contains(&connection) {
                queue.push_back(connection);
            }
        }
    }

    visited.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut graph = parse(input);

    for _ in 0..3 {
        let [a, b] = find_most_common_connection(&graph);
        graph.get_mut(&a).unwrap().remove(b);
        graph.get_mut(&b).unwrap().remove(a);
    }

    let size = calculate_graph_size(&graph);
    Some(size * (graph.len() - size))
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
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
