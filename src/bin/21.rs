use std::collections::{HashSet, VecDeque};

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
    let grid = input
        .trim()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = find_start(&grid);

    let mut visited: HashSet<(usize, (usize, usize))> = HashSet::new();
    visited.insert((0, start));

    let mut queue: VecDeque<(usize, (usize, usize))> = VecDeque::new();
    queue.push_back((0, start));

    while let Some((steps, position)) = queue.pop_front() {
        if steps == steps_to_take {
            break;
        }

        for neighbor in neighbors(position, &grid) {
            let new = (steps + 1, neighbor);

            if !visited.contains(&new) {
                queue.push_back(new);
                visited.insert(new);
            }
        }
    }

    let unique: HashSet<(usize, usize)> = HashSet::from_iter(queue.into_iter().map(|(_, pos)| pos));

    Some(unique.len() + 1)
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 64)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
