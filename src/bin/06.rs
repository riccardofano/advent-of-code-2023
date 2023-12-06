advent_of_code::solution!(6);

fn parse_line(line: &str) -> (&str, Vec<usize>) {
    let (info, nums) = line.split_once(": ").unwrap();
    let nums = nums
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<usize>, _>>()
        .unwrap();

    (info, nums)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut lines = input.trim().lines();
    let (_time_header, times) = parse_line(lines.next().unwrap());
    let (_distance_header, distances) = parse_line(lines.next().unwrap());

    let mut ways_to_win: Vec<usize> = vec![0; times.len()];
    for ((race, &time), &record) in times.iter().enumerate().zip(distances.iter()) {
        for millisecond in 1..time {
            let millimeters = millisecond * (time - millisecond);
            if millimeters > record {
                ways_to_win[race] += 1;
            }
        }
    }

    Some(ways_to_win.iter().product::<usize>())
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
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
