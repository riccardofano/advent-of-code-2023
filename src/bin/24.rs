advent_of_code::solution!(24);

#[derive(Debug)]
struct Hailstone {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    m: f64,
}

impl Hailstone {
    fn parse_line(line: &str) -> Self {
        let (positions, velocities) = line.split_once(" @ ").unwrap();

        let positions = positions
            .split(", ")
            .map(|n| n.trim().parse::<f64>().unwrap())
            .collect::<Vec<f64>>();
        let velocities = velocities
            .split(", ")
            .map(|n| n.trim().parse::<f64>().unwrap())
            .collect::<Vec<f64>>();

        Self {
            x: positions[0],
            y: positions[1],
            z: positions[2],
            vx: velocities[0],
            vy: velocities[1],
            vz: velocities[2],
            m: velocities[1] / velocities[0],
        }
    }

    fn find_intersection(&self, other: &Hailstone) -> Option<(f64, f64)> {
        let det = self.vx * other.vy - self.vy * other.vx;
        if det == 0.0 {
            return None;
        }

        let t = ((other.x - self.x) * other.vy - (other.y - self.y) * other.vx) / det;

        let x_intesection = self.x + self.vx * t;
        let y_intesection = self.y + self.vy * t;

        Some((x_intesection, y_intesection))
    }

    fn is_future_intersection(&self, other: &Hailstone, x_intersection: f64) -> bool {
        let time_a = (x_intersection - self.x) / self.vx;
        let time_b = (x_intersection - other.x) / other.vx;

        time_a > 0.0 && time_b > 0.0
    }
}

fn solve(input: &str, least: f64, most: f64) -> Option<usize> {
    let hailstones = input
        .trim()
        .lines()
        .map(Hailstone::parse_line)
        .collect::<Vec<_>>();

    let mut valid = 0;
    for (i, hailstone) in hailstones.iter().enumerate() {
        for other in hailstones[i + 1..].iter() {
            if let Some((x_intersection, y_intersection)) = hailstone.find_intersection(other) {
                if x_intersection >= least
                    && y_intersection >= least
                    && x_intersection <= most
                    && y_intersection <= most
                    && hailstone.is_future_intersection(other, x_intersection)
                {
                    valid += 1;
                }
            }
        }
    }

    Some(valid)
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 200000000000000.0, 400000000000000.0)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve(
            &advent_of_code::template::read_file("examples", DAY),
            7.0,
            27.0,
        );
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
