advent_of_code::solution!(9);

fn differences(original: &[isize]) -> Vec<isize> {
    let mut differences = vec![0; original.len() - 1];
    for i in 0..original.len() - 1 {
        let difference = original[i + 1] - original[i];
        differences[i] = difference;
    }

    differences
}

fn recurse(original: &[isize]) -> isize {
    if original.iter().all(|&n| n == 0) {
        return 0;
    }

    let diffs = differences(original);
    let new = recurse(&diffs);

    new + original.last().unwrap()
}

pub fn part_one(input: &str) -> Option<isize> {
    let result: isize = input
        .trim()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<isize>, _>>()
                .unwrap()
        })
        .map(|nums| recurse(&nums))
        .sum();

    Some(result)
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
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
