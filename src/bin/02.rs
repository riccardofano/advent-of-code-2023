advent_of_code::solution!(2);

fn is_game_possible(sets: &[&str]) -> bool {
    for set in sets.iter() {
        for cube in set.split(", ") {
            let Some((number, color)) = cube.split_once(' ') else {
                continue;
            };

            dbg!(number, color);
            let number = number.parse::<u32>().unwrap();
            if (color == "red" && number > 12)
                || (color == "green" && number > 13)
                || (color == "blue" && number > 14)
            {
                return false;
            }
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut ids_sum = 0;

    for game in input.lines() {
        let (game_id, rest) = game.split_once(": ").unwrap();
        let id = game_id
            .strip_prefix("Game ")
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let sets = rest.split(';').map(|s| s.trim()).collect::<Vec<_>>();
        if is_game_possible(&sets) {
            ids_sum += id;
        }
    }

    Some(ids_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;

    for game in input.lines() {
        let (_, rest) = game.split_once(": ").unwrap();
        let sets = rest.split(';').map(|s| s.trim()).collect::<Vec<_>>();

        let mut max_reds = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for set in sets.iter() {
            for cube in set.split(", ") {
                let Some((number, color)) = cube.split_once(' ') else {
                    continue;
                };

                let number = number.parse::<u32>().unwrap();
                match color {
                    "red" if number > max_reds => max_reds = number,
                    "green" if number > max_green => max_green = number,
                    "blue" if number > max_blue => max_blue = number,
                    _ => {}
                }
            }
        }

        sum += max_reds * max_green * max_blue;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
