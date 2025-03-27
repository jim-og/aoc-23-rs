use std::collections::HashMap;

use crate::parser;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<String> {
    parser::test_input(input)
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<String>) -> u32 {
    let lookup_1 = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    solve(input, lookup_1)
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<String>) -> u32 {
    let lookup_2 = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    solve(input, lookup_2)
}

fn solve(input: &Vec<String>, lookup: HashMap<&str, i32>) -> u32 {
    let mut sum = 0;
    for line in input {
        let mut digits = Vec::new();

        for index in 0..line.len() {
            for entry in lookup.keys() {
                if line.len() >= index + entry.len() && &&line[index..index + entry.len()] == entry
                {
                    digits.push(lookup.get(entry).unwrap());
                }
            }
        }

        sum += format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
            .parse::<u32>()
            .unwrap();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn part_1() {
        assert_eq!(
            part1(&vec![
                "1abc2".to_string(),
                "pqr3stu8vwx".to_string(),
                "a1b2c3d4e5f".to_string(),
                "treb7uchet".to_string()
            ]),
            142
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            part2(&parser::test_input(
                "nqninenmvnpsz874
                8twofpmpxkvvdnpdnlpkhseven4ncgkb
                six8shdkdcdgseven8xczqrnnmthreecckfive
                qlcnz54dd75nine7jfnlfgz
                7vrdhggdkqbnltlgpkkvsdxn2mfpghkntzrhtjgtxr
                cdhmktwo6kjqbprvfour8
                ninekkvkeight9three",
            )),
            493
        )
    }

    #[test]
    fn mainline() {
        let input = parser::load_input(1);
        assert_eq!(part1(&input), 56397);
        assert_eq!(part2(&input), 55701);
    }
}
