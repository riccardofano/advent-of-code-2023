advent_of_code::solution!(10);

fn build_grid(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect()
}

fn find_start(grid: &[Vec<char>]) -> (usize, usize) {
    for (i, line) in grid.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == 'S' {
                return (j, i);
            }
        }
    }
    panic!("Expected input to have a starting position marked");
}

fn is_valid_position((x, y): (usize, usize), grid: &[Vec<char>]) -> bool {
    x <= grid[0].len() && y <= grid.len()
}

fn neighbors((x, y): (usize, usize), grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    [
        (x.wrapping_sub(1), y),
        (x + 1, y),
        (x, y.wrapping_sub(1)),
        (x, y + 1),
    ]
    .into_iter()
    .filter(|&p| is_valid_position(p, grid))
    .collect()
}

fn pipe_connections(
    (x, y): (usize, usize),
    grid: &[Vec<char>],
) -> Option<((usize, usize), (usize, usize))> {
    if !is_valid_position((x, y), grid) {
        return None;
    }

    match grid[y][x] {
        '|' => Some(((x, y.wrapping_sub(1)), (x, y + 1))),
        '-' => Some(((x.wrapping_sub(1), y), (x + 1, y))),
        'L' => Some(((x, y.wrapping_sub(1)), (x + 1, y))),
        'J' => Some(((x.wrapping_sub(1), y), (x, y.wrapping_sub(1)))),
        '7' => Some(((x.wrapping_sub(1), y), (x, y + 1))),
        'F' => Some(((x, y + 1), (x + 1, y))),
        '.' | 'S' => None,
        c => panic!("Unknown character {c:?}"),
    }
}

fn count_pipes(start: (usize, usize), grid: &[Vec<char>]) -> usize {
    let mut current = start;

    for neighbor in neighbors(start, grid) {
        let Some((conn_1, conn_2)) = pipe_connections(neighbor, grid) else {
            continue;
        };

        if conn_1 == start || conn_2 == start {
            current = neighbor;
            break;
        }
    }

    let mut pipes = vec![start];
    while grid[current.1][current.0] != 'S' {
        let Some((conn_1, conn_2)) = pipe_connections(current, grid) else {
            continue;
        };

        let next = if Some(&conn_1) == pipes.last() {
            conn_2
        } else {
            conn_1
        };
        pipes.push(current);
        current = next;
    }

    pipes.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = build_grid(input);
    let start = find_start(&grid);

    let pipes = count_pipes(start, &grid);
    Some(pipes / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
