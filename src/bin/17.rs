use std::collections::BinaryHeap;

use rustc_hash::FxHashMap;

advent_of_code::solution!(17);

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    cost: usize,
    position: (usize, usize),
    direction: (isize, isize),
    steps_in_same_direction: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(grid: &[&[u8]]) -> Option<usize> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut distances = FxHashMap::default();

    let start = (0, 0);
    let end = (rows - 1, cols - 1);

    let mut heap = BinaryHeap::new();

    heap.push(State {
        cost: 0,
        position: start,
        direction: DIRECTIONS[3],
        steps_in_same_direction: 1,
    });

    while let Some(state) = heap.pop() {
        let State {
            cost,
            position,
            direction,
            steps_in_same_direction,
        } = state;

        if position == end {
            return Some(cost);
        }

        if cost
            > *distances
                .get(&(position, direction, steps_in_same_direction))
                .unwrap_or(&usize::MAX)
        {
            continue;
        }

        for next_direction in DIRECTIONS {
            let steps = if direction == next_direction {
                steps_in_same_direction + 1
            } else {
                1
            };

            // Can't go in the same direction more than 3 times
            if steps > 3 {
                continue;
            }

            // Can't go backwards
            if direction == (-next_direction.0, -next_direction.1) {
                continue;
            }

            let next_row = position.0.wrapping_add_signed(next_direction.0);
            let next_col = position.1.wrapping_add_signed(next_direction.1);

            // Don't exit the grid
            if next_row >= rows || next_col >= cols {
                continue;
            }

            let key = ((next_row, next_col), next_direction, steps);
            let next_cost = cost + (grid[next_row][next_col] - b'0') as usize;

            if next_cost < *distances.get(&key).unwrap_or(&usize::MAX) {
                heap.push(State {
                    cost: next_cost,
                    position: (next_row, next_col),
                    direction: next_direction,
                    steps_in_same_direction: steps,
                });

                distances.insert(key, next_cost);
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = input
        .trim()
        .lines()
        .map(|l| l.as_bytes())
        .collect::<Vec<&[u8]>>();

    shortest_path(&grid)
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
