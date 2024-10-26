use num_integer::lcm;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
enum NodeType {
    Starting,
    Normal,
    Terminating,
}

struct Node {
    id: String,
    left: String,
    right: String,
    node_type: NodeType,
}

impl Node {
    pub fn new(input: &str) -> Self {
        let split = input.split_whitespace().collect::<Vec<&str>>();
        let id = split
            .first()
            .expect("Expected a node id at the start of the input");
        let left = split
            .get(2)
            .expect("Expected a left instruction")
            .trim_matches(['(', ',']);
        let right = split
            .get(3)
            .expect("Expected a right instruction")
            .trim_matches(')');
        let node_type = match id
            .chars()
            .nth(2)
            .expect("Expected id to have a 3rd character")
        {
            'A' => NodeType::Starting,
            'Z' => NodeType::Terminating,
            _ => NodeType::Normal,
        };

        Node {
            id: id.to_string(),
            left: left.to_string(),
            right: right.to_string(),
            node_type,
        }
    }
}

fn part_1(nodes: &HashMap<String, Node>, instructions: &str) -> usize {
    let mut steps = 0;
    let mut step = "AAA";
    while step != "ZZZ" {
        let instruction = instructions
            .chars()
            .nth(steps % instructions.len())
            .expect("Expected");
        let node = nodes.get(step).expect("Couldn't find node");
        if instruction == 'L' {
            step = &node.left;
        } else {
            step = &node.right;
        }
        steps += 1;
    }
    steps
}

fn part_2(nodes: &HashMap<String, Node>, instructions: &str) -> usize {
    let starting_nodes = nodes
        .values()
        .filter(|n| n.node_type == NodeType::Starting)
        .collect::<Vec<&Node>>();

    // Consider the number of steps in each path separately
    // then find the lowest common multiple.
    let mut path_steps = Vec::new();
    for starting_node in starting_nodes {
        let mut steps = 0;
        let mut node = starting_node;
        while node.node_type != NodeType::Terminating {
            let instruction = instructions
                .chars()
                .nth(steps % instructions.len())
                .expect("Expected instruction");
            node = match instruction {
                'L' => nodes.get(&node.left).expect("Expected to find a node"),
                _ => nodes.get(&node.right).expect("Expected to find a node"),
            };
            steps += 1;
        }
        path_steps.push(steps);
    }

    path_steps.iter().fold(1, |a, b| lcm(a, *b))
}

pub fn day08(input: Vec<String>) -> (String, String) {
    let result_1 = day08_1(&input);
    let result_2 = day08_2(&input);
    (result_1.to_string(), result_2.to_string())
}

fn parse_input(input: &Vec<String>) -> (HashMap<String, Node>, String) {
    let mut instructions = String::new();
    let mut nodes = HashMap::new();
    for line in input {
        if instructions.is_empty() {
            instructions = line.to_string();
            continue;
        }
        if line.is_empty() {
            continue;
        }
        let node = Node::new(line);
        nodes.insert(node.id.clone(), node);
    }
    (nodes, instructions)
}

fn day08_1(input: &Vec<String>) -> String {
    let (nodes, instructions) = parse_input(input);
    format!("{}", part_1(&nodes, &instructions))
}

fn day08_2(input: &Vec<String>) -> String {
    let (nodes, instructions) = parse_input(input);
    format!("{}", part_2(&nodes, &instructions))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn example_part_1() {
        let result_1 = day08_1(&vec![
            "RL".to_string(),
            "".to_string(),
            "AAA = (BBB, CCC)".to_string(),
            "BBB = (DDD, EEE)".to_string(),
            "CCC = (ZZZ, GGG)".to_string(),
            "DDD = (DDD, DDD)".to_string(),
            "EEE = (EEE, EEE)".to_string(),
            "GGG = (GGG, GGG)".to_string(),
            "ZZZ = (ZZZ, ZZZ)".to_string(),
        ]);
        assert_eq!(result_1, "2");

        let result_2 = day08_1(&vec![
            "LLR".to_string(),
            "".to_string(),
            "AAA = (BBB, BBB)".to_string(),
            "BBB = (AAA, ZZZ)".to_string(),
            "ZZZ = (ZZZ, ZZZ)".to_string(),
        ]);
        assert_eq!(result_2, "6");
    }

    #[test]
    fn example_part_2() {
        let result = day08_2(&vec![
            "LR".to_string(),
            "".to_string(),
            "11A = (11B, XXX)".to_string(),
            "11B = (XXX, 11Z)".to_string(),
            "11Z = (11B, XXX)".to_string(),
            "22A = (22B, XXX)".to_string(),
            "22B = (22C, 22C)".to_string(),
            "22C = (22Z, 22Z)".to_string(),
            "22Z = (22B, 22B)".to_string(),
            "XXX = (XXX, XXX)".to_string(),
        ]);
        assert_eq!(result, "6");
    }

    #[test]
    fn mainline() {
        let input = parser::load_input(8);
        let result = day08(input);
        assert_eq!(result.0, "13771");
        assert_eq!(result.1, "13129439557681");
    }
}
