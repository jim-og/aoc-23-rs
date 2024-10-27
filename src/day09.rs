enum Direction {
    Forward,
    Backward,
}

pub fn day09(input: Vec<String>) -> (String, String) {
    let mut part_1 = Vec::new();
    let mut part_2 = Vec::new();
    for line in input {
        let history = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().expect("Failed to parse input"))
            .collect::<Vec<i32>>();
        part_1.push(extrapolate(&history, Direction::Forward));
        part_2.push(extrapolate(&history, Direction::Backward));
    }
    (
        format!("{}", part_1.iter().sum::<i32>()),
        format!("{}", part_2.iter().sum::<i32>()),
    )
}

fn extrapolate(history: &[i32], direction: Direction) -> i32 {
    let first_iter = history.iter();
    let second_iter = history.iter().skip(1);
    let diffs = first_iter
        .zip(second_iter)
        .map(|(current, next)| next - current)
        .collect::<Vec<i32>>();
    let all_zeros = diffs.iter().filter(|diff| **diff == 0).count() == diffs.len();
    match direction {
        Direction::Forward => {
            let last_history = *history.last().expect("Expected history to contain a value");
            if all_zeros {
                last_history
            } else {
                last_history + extrapolate(&diffs, direction)
            }
        }
        Direction::Backward => {
            let first_history = *history
                .first()
                .expect("Expected history to contain a value");
            if all_zeros {
                first_history
            } else {
                first_history - extrapolate(&diffs, direction)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn example_both() {
        let result = day09(vec![
            "0 3 6 9 12 15".to_string(),
            "1 3 6 10 15 21".to_string(),
            "10 13 16 21 30 45".to_string(),
        ]);
        assert_eq!(result.0, "114");
        assert_eq!(result.1, "2");
    }

    #[test]
    fn mainline() {
        let input = parser::load_input(9);
        let result = day09(input);
        assert_eq!(result.0, "1974232246");
        assert_eq!(result.1, "928");
    }
}
