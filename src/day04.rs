use std::collections::{HashMap, HashSet};

pub fn day04(input: Vec<String>) -> (String, String) {
    let mut results = HashMap::new();

    for (index, game) in input.iter().enumerate() {
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

    // Part 1
    let mut points = 0;
    for result in results.values() {
        match result {
            0 => (),
            1 => points += 1,
            _ => points += i32::pow(2, (result - 1).try_into().unwrap()),
        }
    }

    // Part 2
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

    (format!("{}", points), format!("{}", total))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn example_both() {
        let result = day04(parser::test_input(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ));
        assert_eq!(result.0, "13");
        assert_eq!(result.1, "30");
    }

    #[test]
    fn mainline() {
        let input = parser::load_input(4);
        let result = day04(input);
        assert_eq!(result.0, "21558");
        assert_eq!(result.1, "10425665");
    }
}
