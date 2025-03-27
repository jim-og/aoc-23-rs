use std::collections::{HashMap, HashSet};

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> HashMap<usize, usize> {
    let mut results = HashMap::new();

    for (index, game) in input
        .trim()
        .lines()
        .map(|l| l.trim().to_string())
        .enumerate()
    {
        let segments = game.split([':', '|']).collect::<Vec<&str>>();
        let winners = segments[1].split_whitespace().collect::<Vec<&str>>();
        let picks: HashSet<&str> =
            HashSet::from_iter(segments[2].split_whitespace().collect::<Vec<&str>>());

        let mut matches: usize = 0;
        for winner in winners {
            if picks.contains(winner) {
                matches += 1;
            }
        }
        results.insert(index + 1, matches);
    }
    results
}

#[aoc(day4, part1)]
pub fn part1(results: &HashMap<usize, usize>) -> i32 {
    let mut points = 0;
    for result in results.values() {
        match result {
            0 => (),
            1 => points += 1,
            _ => points += i32::pow(2, (result - 1).try_into().unwrap()),
        }
    }
    points
}

#[aoc(day4, part2)]
pub fn part2(results: &HashMap<usize, usize>) -> usize {
    let mut total = results.len();
    let mut stack = Vec::new();
    for result in results.clone() {
        stack.push(result.0);
    }
    while let Some(game) = stack.pop() {
        if let Some(matches) = results.get(&game) {
            for k in game + 1..game + matches + 1 {
                if results.contains_key(&k) {
                    stack.push(k);
                    total += 1;
                }
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    const TEST: &str = "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1_test() {
        assert_eq!(part1(&input_generator(TEST)), 13);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&input_generator(TEST)), 30);
    }

    #[test]
    fn mainline() {
        let input = input_generator(&parser::load_input_string(4));
        assert_eq!(part1(&input), 21558);
        assert_eq!(part2(&input), 10425665);
    }
}
