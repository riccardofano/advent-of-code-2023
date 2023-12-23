use std::collections::HashSet;

advent_of_code::solution!(23);

fn directions((row, col): (usize, usize), grid: &[&[u8]]) -> Vec<(isize, isize)> {
    match grid[row][col] {
        b'>' => vec![(0, 1)],
        b'v' => vec![(1, 0)],
        b'<' => vec![(0, -1)],
        b'^' => vec![(-1, 0)],
        _ => vec![(0, 1), (1, 0), (0, -1), (-1, 0)],
    }
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
    None
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
        assert_eq!(result, None);
    }
}
