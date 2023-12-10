use std::collections::HashSet;

advent_of_code::solution!(10);

type Point = (usize, usize);

fn build_grid(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect()
}

fn find_start(grid: &[Vec<char>]) -> Point {
    for (i, line) in grid.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == 'S' {
                return (j, i);
            }
        }
    }
    panic!("Expected input to have a starting position marked");
}

fn is_valid_position((x, y): Point, grid: &[Vec<char>]) -> bool {
    x <= grid[0].len() && y <= grid.len()
}

fn neighbors((x, y): Point, grid: &[Vec<char>]) -> Vec<Point> {
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

fn pipe_connections((x, y): Point, grid: &[Vec<char>]) -> Option<(Point, Point)> {
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

fn find_loop_pipes(start: Point, grid: &[Vec<char>]) -> Vec<Point> {
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

    pipes
}

fn have_connection((conn_1, conn_2): (Point, Point), to_find: Point) -> bool {
    conn_1 == to_find || conn_2 == to_find
}

fn count_inner_tiles(
    grid: &[Vec<char>],
    pipes: HashSet<Point>,
    start_connections: (Point, Point),
) -> usize {
    let mut inner_tiles = 0;

    for y in 0..grid.len() {
        let mut outside = true;

        for x in 0..grid[0].len() {
            if let Some(pos) = pipes.get(&(x, y)) {
                // `pipe_connections` doesn't know what the shape of the starting tile is
                // so we pass them connections manually now that we know
                let connections = pipe_connections(*pos, grid).unwrap_or(start_connections);
                if have_connection(connections, (x, y + 1)) {
                    outside = !outside;
                }
            } else if !outside {
                inner_tiles += 1;
            }
        }
    }

    inner_tiles
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = build_grid(input);
    let start = find_start(&grid);

    let pipes = find_loop_pipes(start, &grid);
    Some(pipes.len() / 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = build_grid(input);
    let start = find_start(&grid);

    let pipes = find_loop_pipes(start, &grid);
    let start_connections = (pipes[1], pipes[pipes.len() - 1]);
    let pipes_set: HashSet<(usize, usize)> = pipes.into_iter().collect();

    Some(count_inner_tiles(&grid, pipes_set, start_connections))
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
    fn test_part_two_one() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(10));
    }
}
