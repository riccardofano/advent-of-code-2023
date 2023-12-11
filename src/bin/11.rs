advent_of_code::solution!(11);

fn galaxy_positions(
    grid: &[Vec<char>],
    empty_cols: &[usize],
    empty_rows: &[usize],
    factor: usize,
) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &col) in row.iter().enumerate() {
            if col == '#' {
                let x_offset = empty_cols.iter().take_while(|&&c| c < x).count() * factor;
                let y_offset = empty_rows.iter().take_while(|&&r| r < y).count() * factor;
                galaxies.push((x + x_offset, y + y_offset));
            }
        }
    }
    galaxies
}

fn solve(input: &str, factor: usize) -> Option<usize> {
    let grid = input
        .trim()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let empty_rows = (0..grid[0].len())
        .filter(|&x| (0..grid.len()).all(|y| grid[x][y] == '.'))
        .collect::<Vec<_>>();
    let empty_cols = (0..grid.len())
        .filter(|&y| (0..grid[0].len()).all(|x| grid[x][y] == '.'))
        .collect::<Vec<_>>();

    let galaxies = galaxy_positions(&grid, &empty_cols, &empty_rows, factor);

    let result: usize = galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, g)| galaxies[i + 1..].iter().map(move |other| (g, other)))
        .map(|(&galaxy, &other)| galaxy.0.abs_diff(other.0) + galaxy.1.abs_diff(other.1))
        .sum();

    Some(result)
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 1)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 1_000_000 - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = solve(
            &advent_of_code::template::read_file("examples", DAY),
            100 - 1,
        );
        assert_eq!(result, Some(8410));
    }
}
