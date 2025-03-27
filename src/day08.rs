use num_integer::lcm;
use std::collections::HashMap;

use crate::parser;

#[derive(Eq, Hash, PartialEq)]
enum NodeType {
    Starting,
    Normal,
    Terminating,
}

enum Part {
    Part1,
    Part2,
}

struct Node {
    id: String,
    left: String,
    right: String,
    node_type: NodeType,
}

impl Node {
    pub fn new(input: &str, part: &Part) -> Self {
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
        let node_type = match part {
            Part::Part1 => match *id {
                "AAA" => NodeType::Starting,
                "ZZZ" => NodeType::Terminating,
                _ => NodeType::Normal,
            },
            Part::Part2 => match id
                .chars()
                .nth(2)
                .expect("Expected id to have a 3rd character")
            {
                'A' => NodeType::Starting,
                'Z' => NodeType::Terminating,
                _ => NodeType::Normal,
            },
        };

        Node {
            id: id.to_string(),
            left: left.to_string(),
            right: right.to_string(),
            node_type,
        }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<String> {
    parser::test_input(input)
}

#[aoc(day8, part1)]
fn part1(input: &Vec<String>) -> usize {
    let (nodes, instructions) = parse_input(input, Part::Part1);
    solve(&nodes, &instructions)
}

#[aoc(day8, part2)]
fn part2(input: &Vec<String>) -> usize {
    let (nodes, instructions) = parse_input(input, Part::Part2);
    solve(&nodes, &instructions)
}

fn parse_input(input: &Vec<String>, part: Part) -> (HashMap<String, Node>, String) {
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
        let node = Node::new(line, &part);
        nodes.insert(node.id.clone(), node);
    }
    (nodes, instructions)
}

fn solve(nodes: &HashMap<String, Node>, instructions: &str) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use test_case::test_case;

    #[test_case(
        "
        RL
            
        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
        ",
        2
        ;"1"
    )]
    #[test_case(
        "
        LLR
            
        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
        ",
        6
        ;"2"
    )]
    fn example_part_1(input: &str, answer: usize) {
        assert_eq!(part1(&input_generator(input)), answer);
    }

    #[test]
    fn example_part_2() {
        let input = input_generator(
            "LR
          
            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)",
        );
        assert_eq!(part2(&input), 6);
    }

    #[test]
    fn mainline() {
        let input = input_generator(&parser::load_input_string(8));
        assert_eq!(part1(&input), 13771);
        assert_eq!(part2(&input), 13129439557681);
    }
}
