#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().expect("Failed to parse input"))
                .collect()
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &Vec<Vec<i32>>) -> i32 {
    input.iter().map(extrapolate).sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &Vec<Vec<i32>>) -> i32 {
    input
        .iter()
        .map(|history| extrapolate(&history.iter().rev().cloned().collect()))
        .sum()
}

fn extrapolate(history: &Vec<i32>) -> i32 {
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
    fn example() {
        let input = input_generator(
            "0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45",
        );
        assert_eq!(part1(&input), 114);
        assert_eq!(part2(&input), 2);
    }

    #[test]
    fn mainline() {
        let input = input_generator(&parser::load_input_string(9));
        assert_eq!(part1(&input), 1974232246);
        assert_eq!(part2(&input), 928);
    }
}
