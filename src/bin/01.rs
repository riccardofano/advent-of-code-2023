advent_of_code::solution!(1);

fn get_first_num_char(iter: impl Iterator<Item = char>) -> char {
    for char in iter {
        if char.is_numeric() {
            return char;
        }
    }

    unreachable!()
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|l| {
            let first = get_first_num_char(l.chars());
            let last = get_first_num_char(l.chars().rev());
            format!("{first}{last}").parse::<u32>().unwrap()
        })
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
        let result = part_one(include_str!("../../data/examples/01-1.txt"));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(include_str!("../../data/examples/01-2.txt"));
        assert_eq!(result, None);
    }
}
