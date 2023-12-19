use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

advent_of_code::solution!(19);

#[derive(Debug, Clone, Copy)]
struct Part([usize; 4]);

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let properties = s
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .collect::<Vec<_>>();

        let x = properties[0].strip_prefix("x=").unwrap().parse().unwrap();
        let m = properties[1].strip_prefix("m=").unwrap().parse().unwrap();
        let a = properties[2].strip_prefix("a=").unwrap().parse().unwrap();
        let s = properties[3].strip_prefix("s=").unwrap().parse().unwrap();

        Ok(Self([x, m, a, s]))
    }
}

#[derive(Debug)]
struct Rule<'a> {
    condition: (usize, char, usize),
    then: &'a str,
}

#[derive(Debug)]
struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    otherwise: &'a str,
}

fn parse_workflow(value: &str) -> (&str, Workflow<'_>) {
    let (label, rest) = value.split_once('{').unwrap();
    let mut rules = rest
        .strip_suffix('}')
        .unwrap()
        .split(',')
        .collect::<Vec<_>>();

    let otherwise = rules.pop().unwrap();
    let rules = rules
        .into_iter()
        .map(|rule| {
            let (condition, then) = rule.split_once(':').unwrap();

            let mut chars = condition.chars();
            let property = match chars.next().unwrap() {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                c => unreachable!("{c:?}"),
            };
            let op = chars.next().unwrap();
            let num = chars.as_str().parse().unwrap();

            Rule {
                condition: (property, op, num),
                then,
            }
        })
        .collect::<Vec<_>>();

    (label, Workflow { rules, otherwise })
}

fn count_accepted(workflows: &HashMap<&str, Workflow>) -> usize {
    let max_ranges = [(1, 4000), (1, 4000), (1, 4000), (1, 4000)];
    let mut queue = VecDeque::new();
    queue.push_back(("in", max_ranges));

    let mut accepted = 0;

    while let Some((label, mut ranges)) = queue.pop_front() {
        if label == "A" {
            let score = ranges
                .into_iter()
                .fold(1, |acc, range| acc * (range.1 - range.0 + 1));
            accepted += score;
            continue;
        }
        if label == "R" {
            continue;
        }

        let workflow = workflows.get(label).unwrap();
        for Rule { condition, then } in &workflow.rules {
            let (var, op, num) = *condition;
            let mut new_ranges = ranges;

            match op {
                '>' => {
                    new_ranges[var].0 = num + 1;
                    ranges[var].1 = num;
                }
                '<' => {
                    new_ranges[var].1 = num - 1;
                    ranges[var].0 = num;
                }
                other => unreachable!("unknown op: {other:?}"),
            }
            queue.push_back((then, new_ranges))
        }

        queue.push_back((workflow.otherwise, ranges));
    }

    accepted
}

pub fn part_one(input: &str) -> Option<usize> {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows = workflows
        .lines()
        .map(parse_workflow)
        .collect::<HashMap<_, _>>();

    let parts = parts
        .lines()
        .map(Part::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();

    for part in parts.iter() {
        let mut current_workflow = "in";
        'workflow: while current_workflow != "A" || current_workflow != "R" {
            let Some(workflow) = workflows.get(&current_workflow) else {
                break;
            };

            for Rule { condition, then } in &workflow.rules {
                let res = match condition.1 {
                    '>' => part.0[condition.0] > condition.2,
                    '<' => part.0[condition.0] < condition.2,
                    op => panic!("found unknown operation {op:?}"),
                };

                if res {
                    current_workflow = then;
                    continue 'workflow;
                }
            }

            current_workflow = workflow.otherwise;
        }

        match current_workflow {
            "A" => accepted.push(part),
            "R" => rejected.push(part),
            label => eprintln!("Got some other label: {label:?}"),
        }
    }

    Some(
        accepted
            .into_iter()
            .map(|&p| p.0.iter().sum::<usize>())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (workflows, _parts) = input.split_once("\n\n").unwrap();

    let workflows = workflows
        .lines()
        .map(parse_workflow)
        .collect::<HashMap<_, _>>();

    // let mins = [1; 4];
    // let maxs = [4000; 4];

    // let mut count = 0;
    // let possibilities = find_mins_maxs("in", &workflows, &mins, &maxs, &mut count);
    // dbg!(count);

    // Some(possibilities)

    Some(count_accepted(&workflows))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
