advent_of_code::solution!(13);

fn transpose(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut next_grid = vec![vec!['?'; grid.len()]; grid[0].len()];

    for (i, row) in grid.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            next_grid[j][i] = *char;
        }
    }

    next_grid
}

fn find_reflection(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();

    for i in 1..rows {
        let mut is_reflection = true;

        for j in 1..=i {
            if i + j > rows {
                break;
            }
            if grid[i - j] != grid[i + j - 1] {
                is_reflection = false;
                break;
            }
        }
        if is_reflection {
            return i;
        }
    }

    0
}

fn find_reflection_two(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();

    for i in 1..rows {
        let mut found_difference = false;
        let mut is_reflection = true;

        for j in 1..=i {
            if i + j > rows {
                break;
            }
            let a = &grid[i - j];
            let b = &grid[i + j - 1];
            if a != b {
                if !found_difference && has_only_one_difference(a, b) {
                    found_difference = true;
                } else {
                    is_reflection = false;
                    break;
                }
            }
        }
        if is_reflection && found_difference {
            return i;
        }
    }

    0
}

fn has_only_one_difference(a: &[char], b: &[char]) -> bool {
    let mut differences = 0;
    for i in 0..a.len() {
        if a[i] != b[i] {
            differences += 1;
        }
        if differences > 1 {
            return false;
        }
    }

    differences == 1
}

pub fn part_one(input: &str) -> Option<usize> {
    let sums: usize = input
        .trim()
        .split("\n\n")
        .map(|s| {
            let grid = s
                .lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            let transposed_grid = transpose(&grid);
            let col_reflection = find_reflection(&transposed_grid);

            if col_reflection == 0 {
                find_reflection(&grid) * 100
            } else {
                col_reflection
            }
        })
        .sum();

    Some(sums)
}

pub fn part_two(input: &str) -> Option<usize> {
    let sums: usize = input
        .trim()
        .split("\n\n")
        .map(|s| {
            let grid = s
                .lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            let transposed_grid = transpose(&grid);
            let col_reflection = find_reflection_two(&transposed_grid);

            if col_reflection == 0 {
                find_reflection_two(&grid) * 100
            } else {
                col_reflection
            }
        })
        .sum();

    Some(sums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_one_from_real_input() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(200))
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
