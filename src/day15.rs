#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_len: usize,
}

enum Operation {
    Insert(usize),
    Remove,
}

fn hash(label: &str) -> usize {
    label
        .chars()
        .fold(0, |acc, c| (acc + c as usize) * 17 % 256)
}

struct Parser {
    steps: Vec<String>,
    init_seq: Vec<(String, Operation)>,
}

impl Parser {
    fn hash_score(&self) -> usize {
        self.steps.iter().fold(0, |acc, step| acc + hash(step))
    }

    fn hash_mapper(&self) -> Vec<Vec<Lens>> {
        let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

        for (label, operation) in &self.init_seq {
            let box_val = hash(label);
            let bx = boxes
                .get_mut(box_val)
                .expect("Expected a box at this index");
            match operation {
                Operation::Insert(focal_len) => {
                    if let Some(lens) = bx.iter_mut().find(|lens| lens.label == *label) {
                        lens.focal_len = *focal_len;
                    } else {
                        bx.push(Lens {
                            label: label.clone(),
                            focal_len: *focal_len,
                        });
                    }
                }
                Operation::Remove => {
                    if let Some(index) = bx.iter().position(|lens| lens.label == *label) {
                        bx.remove(index);
                    }
                }
            }
        }

        boxes
    }
}

trait LensBox {
    fn focusing_power(&self) -> usize;
}

impl LensBox for Vec<Vec<Lens>> {
    fn focusing_power(&self) -> usize {
        self.iter().enumerate().fold(0, |acc, (bx, lenses)| {
            acc + lenses
                .iter()
                .enumerate()
                .map(|(slot, lens)| (slot + 1) * lens.focal_len)
                .sum::<usize>()
                * (bx + 1)
        })
    }
}

fn initializer(step: &str) -> Option<(String, Operation)> {
    if let Some(label) = step.strip_suffix('-') {
        Some((label.to_string(), Operation::Remove))
    } else if let Some((label, focal_len)) = step.split_once('=') {
        Some((
            label.to_string(),
            Operation::Insert(
                focal_len
                    .parse::<usize>()
                    .expect("Unable to parse focal length"),
            ),
        ))
    } else {
        // Steps which are valid for Part 1 may not be valid for Part 2.
        // Return None so they can be discarded for Part 2.
        None
    }
}

#[aoc_generator(day15)]
fn parse(input: &str) -> Parser {
    Parser {
        steps: input.trim().split(',').map(|s| s.to_string()).collect(),
        init_seq: input.trim().split(',').filter_map(initializer).collect(),
    }
}

#[aoc(day15, part1)]
fn part1(input: &Parser) -> usize {
    input.hash_score()
}

#[aoc(day15, part2)]
fn part2(input: &Parser) -> usize {
    input.hash_mapper().focusing_power()
}

#[cfg(test)]
mod tests {
    use crate::parser;

    use super::*;
    use test_case::test_case;

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test_case(
        "HASH", 
        52
        ;"e1"
    )]
    #[test_case(
        EXAMPLE,
        1320
        ;"e2"
    )]
    #[test_case(
        "qp", 
        1
        ;"e3"
    )]
    fn part1_example(input: &str, want: usize) {
        assert_eq!(part1(&parse(input)), want);
    }

    #[test_case(
        EXAMPLE,
        145
        ;"e1"
    )]
    fn part2_example(input: &str, want: usize) {
        assert_eq!(part2(&parse(input)), want);
    }

    #[test]
    fn mainline() {
        let input = &parser::load_input_string(15);
        assert_eq!(part1(&parse(input)), 522547);
        assert_eq!(part2(&parse(input)), 229271);
    }
}
