use std::collections::{HashMap, HashSet};

enum Point {
    Period,
    Symbol(char),
    Part(u32),
}

fn process_part(
    schematic: &mut HashMap<(usize, usize), Point>,
    row: usize,
    col: usize,
    part: &mut Vec<char>,
) {
    let number = part
        .iter()
        .collect::<String>()
        .parse::<u32>()
        .expect("Unable to parse part number");

    for i in 0..part.len() {
        schematic.insert((row, col - i), Point::Part(number));
    }
    part.clear();
}

pub fn day03(input: Vec<String>) -> (String, String) {
    let mut schematic = HashMap::new();

    // Populate schematic
    for (row, line) in input.iter().enumerate() {
        let mut part = Vec::new();
        for (col, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    if !part.is_empty() {
                        process_part(&mut schematic, row, col - 1, &mut part);
                    }
                    schematic.insert((row, col), Point::Period);
                }
                _ => {
                    if c.is_numeric() {
                        part.push(c);
                    } else {
                        if !part.is_empty() {
                            process_part(&mut schematic, row, col - 1, &mut part);
                        }
                        schematic.insert((row, col), Point::Symbol(c));
                    }
                }
            }
        }

        // Handle number at the end of a row
        if !part.is_empty() {
            process_part(&mut schematic, row, line.len() - 1, &mut part);
        }
    }

    // Find symbols in the schematic and sum adjacent parts
    let mut part_1_parts = Vec::new();
    let mut part_2_sum = 0;
    for ((row, col), point) in &schematic {
        if let Point::Symbol(s) = point {
            let mut adjacent_parts = HashSet::new();
            for r in row - 1..row + 2 {
                for c in col - 1..col + 2 {
                    if let Some(Point::Part(n)) = schematic.get(&(r, c)) {
                        adjacent_parts.insert(*n);
                    }
                }
            }
            for part in &adjacent_parts {
                part_1_parts.push(*part);
            }
            if *s == '*' && adjacent_parts.len() == 2 {
                part_2_sum += adjacent_parts.iter().product::<u32>();
            }
        }
    }

    (
        format!("{}", part_1_parts.iter().sum::<u32>()),
        format!("{}", part_2_sum),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn example() {
        let result = day03(parser::test_input(
            "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..",
        ));
        assert_eq!(result.0, "4361");
        assert_eq!(result.1, "467835");
    }

    #[test]
    fn mainline() {
        let input = parser::load_input(3);
        let result = day03(input);
        assert_eq!(result.0, "532331");
        assert_eq!(result.1, "82301120");
    }
}
