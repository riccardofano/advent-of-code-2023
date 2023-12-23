use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(23);

// The points connected to a point and their distance
type Distances = HashMap<(usize, usize), Vec<((usize, usize), usize)>>;

fn directions((row, col): (usize, usize), grid: &[&[u8]]) -> Vec<(isize, isize)> {
    match grid[row][col] {
        b'>' => vec![(0, 1)],
        b'v' => vec![(1, 0)],
        b'<' => vec![(0, -1)],
        b'^' => vec![(-1, 0)],
        _ => vec![(0, 1), (1, 0), (0, -1), (-1, 0)],
    }
}

fn neighbors_two((row, col): (usize, usize), grid: &[&[u8]]) -> Vec<(usize, usize)> {
    [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .into_iter()
        .map(|(dy, dx)| (row.wrapping_add_signed(dy), col.wrapping_add_signed(dx)))
        .filter(|&(new_row, new_col)| is_valid_position((new_row, new_col), grid))
        .collect()
}

fn is_valid_position((row, col): (usize, usize), grid: &[&[u8]]) -> bool {
    row < grid.len() && col < grid[0].len() && grid[row][col] != b'#'
}

fn longest_path(
    current: (usize, usize),
    grid: &[&[u8]],
    path: &mut Vec<(usize, usize)>,
    visited: &mut HashSet<(usize, usize)>,
    best: &mut usize,
) {
    let rows = grid.len();
    let cols = grid[0].len();

    if current == (rows - 1, cols - 2) {
        *best = (*best).max(path.len());
        return;
    }

    for direction in directions(current, grid) {
        let new_spot = (
            current.0.wrapping_add_signed(direction.0),
            current.1.wrapping_add_signed(direction.1),
        );

        if new_spot.0 >= rows
            || new_spot.1 >= cols
            || grid[new_spot.0][new_spot.1] == b'#'
            || visited.contains(&new_spot)
        {
            continue;
        }

        visited.insert(new_spot);
        path.push(new_spot);

        longest_path(new_spot, grid, path, visited, best);

        path.pop();
        visited.remove(&new_spot);
    }
}

fn find_junctions(grid: &[&[u8]]) -> HashSet<(usize, usize)> {
    let mut junctions = HashSet::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            if col != b'#' && neighbors_two((i, j), grid).len() > 2 {
                junctions.insert((i, j));
            }
        }
    }

    junctions
}

fn distances_between_junctions(junctions: HashSet<(usize, usize)>, grid: &[&[u8]]) -> Distances {
    let mut distances = HashMap::from_iter(junctions.iter().map(|p| (*p, vec![])));

    for junction in junctions.iter() {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        queue.push_back(*junction);
        seen.insert(*junction);

        let mut distance = 0;

        while !queue.is_empty() {
            distance += 1;
            let mut next_queue = VecDeque::new();

            while let Some(cell) = queue.pop_front() {
                let neighbors = neighbors_two(cell, grid);
                for neighbor in neighbors.iter() {
                    if seen.contains(neighbor) {
                        continue;
                    }

                    if junctions.contains(neighbor) {
                        (*distances.get_mut(junction).unwrap()).push((*neighbor, distance));
                    } else {
                        next_queue.push_back(*neighbor);
                    }
                    seen.insert(*neighbor);
                }
            }

            queue = next_queue;
        }
    }

    distances
}

fn longest_path_two(
    current: (usize, usize),
    end: (usize, usize),
    graph: &Distances,
    visited: &mut HashSet<(usize, usize)>,
    distance: usize,
    best: &mut usize,
) {
    if current == end {
        *best = (*best).max(distance);
    }

    for (neighbor, path_len) in graph.get(&current).unwrap() {
        if !visited.contains(neighbor) {
            visited.insert(*neighbor);
            longest_path_two(*neighbor, end, graph, visited, distance + path_len, best);
            visited.remove(neighbor);
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = input
        .trim()
        .lines()
        .map(|l| l.as_bytes())
        .collect::<Vec<_>>();

    let mut path = Vec::new();
    let mut visited = HashSet::new();
    let mut best = 0;
    longest_path((0, 1), &grid, &mut path, &mut visited, &mut best);

    Some(best)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = input
        .trim()
        .lines()
        .map(|l| l.as_bytes())
        .collect::<Vec<_>>();

    let start = (0, 1);
    let end = (grid.len() - 1, grid.len() - 2);

    let mut junctions = find_junctions(&grid);
    junctions.insert(start);
    junctions.insert(end);

    let distances = distances_between_junctions(junctions, &grid);

    let mut visited = HashSet::new();
    let mut best = 0;
    longest_path_two(start, end, &distances, &mut visited, 0, &mut best);

    Some(best)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
