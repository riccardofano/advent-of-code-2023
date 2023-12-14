advent_of_code::solution!(14);

fn fill_cells(
    grid: &mut [Vec<char>],
    col: usize,
    row: usize,
    boulder_amount: usize,
    empty_amount: usize,
) -> (usize, usize) {
    let beginning = row - boulder_amount - empty_amount;
    let after_boulders = beginning + boulder_amount;

    for b in 0..boulder_amount {
        grid[beginning + b][col] = 'O';
    }
    for s in 0..empty_amount {
        grid[after_boulders + s][col] = '.';
    }

    (0, 0)
}

fn move_rocks_north(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut next_grid = grid.to_vec();

    for col in 0..grid[0].len() {
        let mut boulders = 0;
        let mut empties = 0;

        for row in 0..grid.len() {
            match grid[row][col] {
                '.' => empties += 1,
                'O' => boulders += 1,
                '#' => {
                    fill_cells(&mut next_grid, col, row, boulders, empties);
                    next_grid[row][col] = '#';

                    boulders = 0;
                    empties = 0;
                }
                c => panic!("Got bad character: {c:?}"),
            }

            fill_cells(&mut next_grid, col, grid.len(), boulders, empties);
        }
    }

    next_grid
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

pub fn part_one(input: &str) -> Option<usize> {
    let grid = input
        .trim()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let moved = move_rocks_north(&grid);
    // print_grid(&moved);
    let load = boulder_load(&moved);

    Some(load)
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
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
