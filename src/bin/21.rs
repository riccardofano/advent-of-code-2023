use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(21);

fn find_start(grid: &[Vec<char>]) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            if col == 'S' {
                return (i, j);
            }
        }
    }

    unreachable!()
}

fn neighbors((row, col): (usize, usize), grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    vec![
        (row.wrapping_sub(1), col),
        (row + 1, col),
        (row, col.wrapping_sub(1)),
        (row, col + 1),
    ]
    .into_iter()
    .filter(|(i, j)| *i < grid.len() && *j < grid[0].len())
    .filter(|(i, j)| grid[*i][*j] != '#')
    .collect()
}

fn solve(input: &str, steps_to_take: usize) -> Option<usize> {
    let visited = bfs(input);
    let after_steps = visited
        .values()
        .filter(|&&v| v <= steps_to_take && v % 2 == 0)
        .count();

    Some(after_steps)
}

fn bfs(input: &str) -> HashMap<(usize, usize), usize> {
    let grid = input
        .trim()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = find_start(&grid);

    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    let mut queue: VecDeque<(usize, (usize, usize))> = VecDeque::new();
    queue.push_back((0, start));

    while let Some((steps, position)) = queue.pop_front() {
        if visited.contains_key(&position) {
            continue;
        }
        visited.insert(position, steps);

        for neighbor in neighbors(position, &grid) {
            if !visited.contains_key(&neighbor) {
                queue.push_back((steps + 1, neighbor));
            }
        }
    }

    visited
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 64)
}

// https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
pub fn part_two(input: &str) -> Option<usize> {
    let visited = bfs(input);

    let even_corners = visited.values().filter(|&&v| v > 65 && v % 2 == 0).count();
    let odd_corners = visited.values().filter(|&&v| v > 65 && v % 2 == 1).count();

    let n = (26501365 - (131 / 2)) / 131;
    let even = n * n;
    let odd = (n + 1) * (n + 1);

    let even_visited = visited.values().filter(|&&v| v % 2 == 0).count();
    let odd_visited = visited.values().filter(|&&v| v % 2 == 1).count();

    Some(odd * odd_visited + even * even_visited - ((n + 1) * odd_corners) + (n * even_corners))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, Some(16));
    }
}
