use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(16);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self, current: Position) -> Position {
        match self {
            Direction::Up => (current.0 - 1, current.1),
            Direction::Down => (current.0 + 1, current.1),
            Direction::Left => (current.0, current.1 - 1),
            Direction::Right => (current.0, current.1 + 1),
        }
    }
}

type Position = (isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    position: Position,
    direction: Direction,
}

impl Beam {
    fn new(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }

    fn next(&mut self, grid: &[&[u8]]) -> Option<(Beam, Beam)> {
        let (y, x) = self.position;

        self.direction = match grid[y as usize][x as usize] {
            b'.' => self.direction,
            b'/' => match self.direction {
                Direction::Down => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Right,
                Direction::Left => Direction::Down,
            },
            b'\\' => match self.direction {
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Up,
            },
            b'-' => match self.direction {
                Direction::Left | Direction::Right => self.direction,
                Direction::Up | Direction::Down => {
                    return Some((
                        Beam::new(self.position, Direction::Left),
                        Beam::new(self.position, Direction::Right),
                    ))
                }
            },
            b'|' => match self.direction {
                Direction::Up | Direction::Down => self.direction,
                Direction::Left | Direction::Right => {
                    return Some((
                        Beam::new(self.position, Direction::Up),
                        Beam::new(self.position, Direction::Down),
                    ))
                }
            },
            _ => unreachable!(),
        };

        self.position = self.direction.next(self.position);
        None
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = input
        .trim()
        .lines()
        .map(|l| l.as_bytes())
        .collect::<Vec<_>>();

    let mut beams: VecDeque<Beam> = VecDeque::new();
    beams.push_back(Beam::new((0, 0), Direction::Right));

    let mut energized_cells: HashSet<Position> = HashSet::new();
    let mut seen: HashSet<Beam> = HashSet::new();

    while let Some(mut beam) = beams.pop_front() {
        loop {
            let current_pos = beam.position;

            if seen.contains(&beam) {
                break;
            }

            println!("{current_pos:?} going {:?}", beam.direction);
            if current_pos.0 < 0
                || current_pos.0 >= grid.len() as isize
                || current_pos.1 < 0
                || current_pos.1 >= grid[0].len() as isize
            {
                break;
            }
            println!("added");
            energized_cells.insert(current_pos);

            seen.insert(beam);

            if let Some((beam1, beam2)) = beam.next(&grid) {
                beams.push_back(beam1);
                beams.push_back(beam2);
                break;
            }
        }
    }

    Some(energized_cells.len())
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
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
