use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(20);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, PartialEq)]
enum NodeKind {
    FlipFlop(bool),
    Conjunction(bool),
    Broadcast,
}

#[derive(Debug, Clone)]
struct Node<'a> {
    name: &'a str,
    kind: NodeKind,
    inputs: Option<HashMap<&'a str, Pulse>>,
    destinations: Vec<&'a str>,
}

impl<'a> Node<'a> {
    fn parse_line(line: &'a str) -> Self {
        let (info, destinations) = line.split_once(" -> ").unwrap();
        let destinations = destinations.split(", ").collect::<Vec<_>>();

        let (name, kind) = match info.as_bytes().first().unwrap() {
            b'%' => (info.strip_prefix('%').unwrap(), NodeKind::FlipFlop(false)),
            b'&' => (
                info.strip_prefix('&').unwrap(),
                NodeKind::Conjunction(false),
            ),
            b'b' => (info, NodeKind::Broadcast),
            _ => unreachable!(),
        };

        let inputs = if matches!(kind, NodeKind::Conjunction(_)) {
            Some(HashMap::new())
        } else {
            None
        };

        Self {
            name,
            kind,
            destinations,
            inputs,
        }
    }

    fn process(
        &mut self,
        sender: &'a str,
        pulse: Pulse,
        queue: &mut VecDeque<(&'a str, &'a str, Pulse)>,
    ) {
        let old_kind = self.kind.clone();

        let next_pulse = match &old_kind {
            NodeKind::FlipFlop(state) => {
                if pulse == Pulse::Low {
                    self.kind = NodeKind::FlipFlop(!state);
                    Some(if *state { Pulse::Low } else { Pulse::High })
                } else {
                    None
                }
            }
            NodeKind::Conjunction(_) => {
                let Some(ref mut inputs) = self.inputs else {
                    unreachable!();
                };
                inputs.insert(sender, pulse);

                if inputs.values().all(|&p| p == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    self.kind = NodeKind::Conjunction(true);
                    Some(Pulse::High)
                }
            }
            NodeKind::Broadcast => Some(pulse),
        };

        if let Some(next_pulse) = next_pulse {
            for dest in &self.destinations {
                queue.push_back((self.name, dest, next_pulse));
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut nodes = input
        .trim()
        .lines()
        .map(Node::parse_line)
        .map(|node| (node.name, node))
        .collect::<HashMap<_, _>>();

    let conjunction_nodes = nodes
        .iter()
        .filter(|(_key, node)| matches!(node.kind, NodeKind::Conjunction(_)))
        .map(|(&key, _)| key.clone())
        .collect::<Vec<_>>();

    for conjunction_node in conjunction_nodes.iter() {
        let mut inputs = HashMap::new();
        for (name, node) in &nodes {
            if node.destinations.contains(conjunction_node) {
                inputs.insert(*name, Pulse::Low);
            }
        }

        let n = nodes.get_mut(conjunction_node).unwrap();
        n.inputs = Some(inputs);
    }

    let mut high_pulses = 0;
    let mut low_pulses = 0;

    let mut queue: VecDeque<(&str, &str, Pulse)> = VecDeque::new();
    for _ in 0..1000 {
        queue.push_back(("button", "broadcaster", Pulse::Low));

        while let Some((sender, label, pulse)) = queue.pop_front() {
            match pulse {
                Pulse::Low => low_pulses += 1,
                Pulse::High => high_pulses += 1,
            }

            if label == "output" {
                continue;
            }

            let Some(node) = nodes.get_mut(label) else {
                continue;
            };
            node.process(sender, pulse, &mut queue);
        }
    }

    Some(high_pulses * low_pulses)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut nodes = input
        .trim()
        .lines()
        .map(Node::parse_line)
        .map(|node| (node.name, node))
        .collect::<HashMap<_, _>>();

    let conjunction_nodes = nodes
        .iter()
        .filter(|(_key, node)| matches!(node.kind, NodeKind::Conjunction(_)))
        .map(|(&key, _)| key.clone())
        .collect::<Vec<_>>();

    let rx_sender = nodes
        .iter()
        .find(|(_, node)| node.destinations.contains(&"rx"))
        .unwrap()
        .0;
    let rx_inputs = nodes
        .iter()
        .filter(|(_, node)| node.destinations.contains(rx_sender))
        .map(|(key, _)| *key)
        .collect::<Vec<_>>();

    for conjunction_node in conjunction_nodes.iter() {
        let mut inputs = HashMap::new();
        for (name, node) in &nodes {
            if node.destinations.contains(conjunction_node) {
                inputs.insert(*name, Pulse::Low);
            }
        }

        let n = nodes.get_mut(conjunction_node).unwrap();
        n.inputs = Some(inputs);
    }

    let mut trigger_presses = HashMap::new();

    let mut queue: VecDeque<(&str, &str, Pulse)> = VecDeque::new();
    for i in 1.. {
        queue.push_back(("button", "broadcaster", Pulse::Low));

        while let Some((sender, label, pulse)) = queue.pop_front() {
            if label == "output" {
                continue;
            }

            let Some(node) = nodes.get_mut(label) else {
                continue;
            };
            node.process(sender, pulse, &mut queue);

            if node.kind == NodeKind::Conjunction(true)
                && rx_inputs.contains(&node.name)
                && trigger_presses.get(node.name).is_none()
            {
                trigger_presses.insert(node.name, i);
            }

            if trigger_presses.len() == rx_inputs.len() {
                return Some(trigger_presses.values().product::<usize>());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(32000000));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11687500));
    }
}
