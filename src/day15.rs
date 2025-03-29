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

#[aoc(day15, part1)]
fn part1(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .fold(0, |acc, step| acc + hash(step))
}

#[aoc(day15, part2)]
fn part2(input: &str) -> usize {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
    let mut steps: Vec<(String, Operation)> = Vec::new();

    for step in input.trim().split(',') {
        if step.ends_with('-') {
            // Remove
            let label = step[..step.len() - 1].to_string();
            steps.push((label, Operation::Remove));
        } else if let Some((label, focal_len)) = step.split_once('=') {
            // Insert
            steps.push((
                label.to_string(),
                Operation::Insert(
                    focal_len
                        .parse::<usize>()
                        .expect("Unable to parse focal length"),
                ),
            ));
        } else {
            panic!("Invalid step")
        }
    }

    for (label, operation) in steps {
        let box_val = hash(&label);
        let bx = boxes
            .get_mut(box_val)
            .expect("Expected a box at this index");
        match operation {
            Operation::Insert(focal_len) => {
                if let Some(lens) = bx.iter_mut().find(|lens| lens.label == label) {
                    lens.focal_len = focal_len;
                } else {
                    bx.push(Lens {
                        label: label,
                        focal_len: focal_len,
                    });
                }
            }
            Operation::Remove => {
                if let Some(index) = bx.iter().position(|lens| lens.label == label) {
                    bx.remove(index);
                }
            }
        }
    }

    boxes.iter().enumerate().fold(0, |acc, (bx, lenses)| {
        acc + lenses
            .iter()
            .enumerate()
            .map(|(slot, lens)| (slot + 1) * lens.focal_len)
            .sum::<usize>()
            * (bx + 1)
    })
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
        assert_eq!(part1(input), want);
    }

    #[test_case(
        EXAMPLE, 
        145
        ;"e1"
    )]
    fn part2_example(input: &str, want: usize) {
        assert_eq!(part2(input), want);
    }

    #[test]
    fn mainline() {
        let input = &parser::load_input_string(15);
        assert_eq!(part1(input), 522547);
        assert_eq!(part2(input), 229271);
    }
}
