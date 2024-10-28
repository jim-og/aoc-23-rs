pub fn day09(input: Vec<String>) -> (String, String) {
    let mut part_1 = Vec::new();
    let mut part_2 = Vec::new();
    for line in input {
        let history = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().expect("Failed to parse input"))
            .collect::<Vec<i32>>();
        part_1.push(extrapolate(&history));
        part_2.push(extrapolate(
            &history.into_iter().rev().collect::<Vec<i32>>(),
        ));
    }
    (
        format!("{}", part_1.iter().sum::<i32>()),
        format!("{}", part_2.iter().sum::<i32>()),
    )
}

fn extrapolate(history: &[i32]) -> i32 {
    let diffs = history
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<i32>>();
    let last_history = *history.last().expect("Expected history to contain a value");
    if diffs.iter().all(|diff| *diff == 0) {
        last_history
    } else {
        last_history + extrapolate(&diffs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn example_both() {
        let result = day09(parser::test_input(
            "0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45",
        ));
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
