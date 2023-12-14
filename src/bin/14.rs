use std::collections::HashMap;

advent_of_code::solution!(14);

fn move_rocks_north(grid: &mut [Vec<char>]) {
    for col in 0..grid[0].len() {
        let mut available = 0;

        for row in 0..grid.len() {
            match grid[row][col] {
                'O' => {
                    if available < row {
                        grid[available][col] = 'O';
                        grid[row][col] = '.';
                    }
                    available += 1;
                }
                '#' => {
                    available = row + 1;
                }
                _ => {}
            }
        }
    }
}

fn move_rocks_west(grid: &mut [Vec<char>]) {
    for row in 0..grid.len() {
        let mut available = 0;

        for col in 0..grid[0].len() {
            match grid[row][col] {
                'O' => {
                    if available < col {
                        grid[row][available] = 'O';
                        grid[row][col] = '.';
                    }
                    available += 1;
                }
                '#' => {
                    available = col + 1;
                }
                _ => {}
            }
        }
    }
}

fn move_rocks_south(grid: &mut [Vec<char>]) {
    for col in 0..grid[0].len() {
        let mut available = grid.len() as isize - 1;

        for row in (0..grid.len()).rev() {
            match grid[row][col] {
                'O' => {
                    if available > row as isize {
                        grid[available as usize][col] = 'O';
                        grid[row][col] = '.';
                    }
                    available -= 1;
                }
                '#' => {
                    available = row as isize - 1;
                }
                _ => {}
            }
        }
    }
}

fn move_rocks_east(grid: &mut [Vec<char>]) {
    for row in 0..grid.len() {
        let mut available = grid[0].len() as isize - 1;

        for col in (0..grid[0].len()).rev() {
            match grid[row][col] {
                'O' => {
                    if available > col as isize {
                        grid[row][available as usize] = 'O';
                        grid[row][col] = '.';
                    }
                    available -= 1;
                }
                '#' => {
                    available = col as isize - 1;
                }
                _ => {}
            }
        }
    }
}

fn boulder_load(grid: &[Vec<char>]) -> usize {
    let mut load = 0;

    for (i, row) in grid.iter().enumerate() {
        for col in row.iter() {
            if *col == 'O' {
                load += grid.len() - i;
            }
        }
    }

    load
}

fn print_grid(grid: &[Vec<char>]) {
    for row in grid.iter() {
        for col in row.iter() {
            print!("{col}");
        }
        println!()
    }
    println!()
}

fn stringify_grid(grid: &[Vec<char>]) -> String {
    let mut str = String::with_capacity(grid[0].len() * grid.len());
    for row in grid.iter() {
        str.push_str(&row.iter().collect::<String>());
    }
    str
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = input
        .trim()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    move_rocks_north(&mut grid);
    let load = boulder_load(&grid);

    Some(load)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = input
        .trim()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut map = HashMap::new();

    let cycles = 1_000_000_000;
    let mut i = 0;

    while i < cycles {
        move_rocks_north(&mut grid);
        move_rocks_west(&mut grid);
        move_rocks_south(&mut grid);
        move_rocks_east(&mut grid);

        let grid_as_str = stringify_grid(&grid);

        if let Some(seen) = map.get(&grid_as_str) {
            i = cycles - (cycles - i) % (i - seen);
        }

        map.insert(grid_as_str, i);
        i += 1;
    }

    Some(boulder_load(&grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
