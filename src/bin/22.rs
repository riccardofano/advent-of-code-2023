use std::{
    collections::{HashSet, VecDeque},
    ops::RangeInclusive,
};

advent_of_code::solution!(22);

#[derive(Debug)]
struct Brick {
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
    z: RangeInclusive<usize>,
}

impl Brick {
    fn parse_line(line: &str) -> Self {
        let (start, end) = line.split_once('~').unwrap();
        let start = start
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let end = end
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        Self {
            x: start[0]..=end[0],
            y: start[1]..=end[1],
            z: start[2]..=end[2],
        }
    }
}

fn find_over_under(bricks: &[Brick]) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut over = vec![vec![]; bricks.len()];
    let mut under = vec![vec![]; bricks.len()];

    let mut xy_plane = [[0; 10]; 10];
    let mut indices = [[usize::MAX; 10]; 10];

    for (i, brick) in bricks.iter().enumerate() {
        let z = brick.z.end() - brick.z.start() + 1;
        let mut top = 0;

        for x in brick.x.clone() {
            for y in brick.y.clone() {
                top = top.max(xy_plane[x][y]);
            }
        }

        let mut previous = usize::MAX;
        for x in brick.x.clone() {
            for y in brick.y.clone() {
                if xy_plane[x][y] == top && indices[x][y] != previous {
                    over[indices[x][y]].push(i);
                    under[i].push(indices[x][y]);
                    previous = indices[x][y];
                }

                xy_plane[x][y] = top + z;
                indices[x][y] = i;
            }
        }
    }

    (over, under)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut bricks = input
        .trim()
        .lines()
        .map(Brick::parse_line)
        .collect::<Vec<_>>();

    bricks.sort_by(|a, b| a.z.start().cmp(b.z.start()));

    let (_, under) = find_over_under(&bricks);
    let not_safe: HashSet<usize> =
        HashSet::from_iter(under.iter().filter(|b| b.len() == 1).map(|b| b[0]));

    Some(bricks.len() - not_safe.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut bricks = input
        .trim()
        .lines()
        .map(Brick::parse_line)
        .collect::<Vec<_>>();

    bricks.sort_by(|a, b| a.z.start().cmp(b.z.start()));

    let (over, under) = find_over_under(&bricks);
    let not_safe: HashSet<usize> =
        HashSet::from_iter(under.iter().filter(|b| b.len() == 1).map(|b| b[0]));

    let mut sum = 0;
    let mut queue = VecDeque::new();
    let mut removed = vec![usize::MAX; under.len()];

    for &index in not_safe.iter() {
        queue.push_back(index);
        removed[index] = index;

        while let Some(current) = queue.pop_front() {
            for &brick in &over[current] {
                if removed[brick] != index && under[brick].iter().all(|&b| removed[b] == index) {
                    sum += 1;
                    removed[brick] = index;
                    queue.push_back(brick);
                }
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
