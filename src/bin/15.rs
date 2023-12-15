use std::array;

advent_of_code::solution!(15);

fn hash(chars: &str) -> usize {
    chars
        .bytes()
        .fold(0, |acc, curr| ((acc + curr as usize) * 17) % 256)
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.trim().split(',').map(hash).sum())
}

type LensBox<'a> = Vec<(&'a str, usize)>;

pub fn part_two(input: &str) -> Option<usize> {
    let mut boxes: [LensBox; 256] = array::from_fn(|_| Vec::new());

    input
        .trim()
        .split(',')
        .for_each(|instruction| match instruction.split_once('-') {
            Some((label, _)) => {
                let lens_box = hash(label);
                if let Some(index) = boxes[lens_box].iter().position(|lens| lens.0 == label) {
                    boxes[lens_box].remove(index);
                }
            }
            None => {
                let (label, focus) = instruction.split_once('=').unwrap();
                let lens_box = hash(label);
                let focus: usize = focus.parse().unwrap();

                match boxes[lens_box].iter().position(|lens| lens.0 == label) {
                    Some(index) => boxes[lens_box][index].1 = focus,
                    None => boxes[lens_box].push((label, focus)),
                }
            }
        });

    let res: usize = boxes
        .iter()
        .enumerate()
        .map(|(i, lens_box)| {
            lens_box
                .iter()
                .enumerate()
                .map(|(j, lens)| (i + 1) * (j + 1) * lens.1)
                .sum::<usize>()
        })
        .sum();

    Some(res)
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
        assert_eq!(result, Some(145));
    }
}
