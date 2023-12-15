advent_of_code::solution!(15);

fn hash(chars: &str) -> usize {
    chars
        .bytes()
        .fold(0, |acc, curr| ((acc + curr as usize) * 17) % 256)
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.trim().split(',').map(hash).sum())
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
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
