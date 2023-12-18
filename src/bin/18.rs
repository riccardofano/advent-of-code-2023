advent_of_code::solution!(18);

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

struct Instruction<'a> {
    direction: usize,
    length: usize,
    color: &'a str,
}

type Point = (isize, isize);

fn parse_line(line: &str) -> Instruction {
    let [direction, length, color]: [&str; 3] = line
        .split_whitespace()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let direction = match direction {
        "U" => 0,
        "D" => 1,
        "L" => 2,
        "R" => 3,
        _ => unreachable!(),
    };

    let length = length.parse().unwrap();
    let color = color.strip_prefix('(').unwrap().strip_suffix(')').unwrap();

    Instruction {
        direction,
        length,
        color,
    }
}

fn find_points(input: &str) -> Vec<Point> {
    let mut current_position: Point = (0, 0);
    let mut points = vec![current_position];

    let instructions = input.trim().lines().map(parse_line).collect::<Vec<_>>();

    for instruction in instructions {
        let direction = DIRECTIONS[instruction.direction];
        let next_y = current_position.0 + (direction.0 * instruction.length as isize);
        let next_x = current_position.1 + (direction.1 * instruction.length as isize);
        current_position = (next_y, next_x);
        points.push(current_position);
    }

    points
}

fn polygon_area(points: &[Point]) -> isize {
    let n = points.len();
    let mut area = 0;

    for i in 0..n {
        let j = (i + 1) % n;
        area += (points[i].1 * points[j].0) - (points[i].0 * points[j].1);
    }

    area / 2
}

fn polygon_perimiter(points: &[Point]) -> isize {
    let n = points.len();

    let mut perimeter = 0;
    for i in 0..n {
        let j = (i + 1) % n;
        perimeter += (points[i].0 - points[j].0).abs() + (points[i].1 - points[j].1).abs();
    }

    perimeter
}

pub fn part_one(input: &str) -> Option<isize> {
    let points = find_points(input);
    let area = polygon_area(&points);
    let perimeter = polygon_perimiter(&points);

    Some(area + (perimeter / 2) + 1)
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
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
