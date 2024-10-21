use std::collections::HashSet;

pub fn day03(input: Vec<String>) -> (String, String) {
    // Find coordinates of symbols
    let mut symbol_coords = HashSet::new();
    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if !c.is_numeric() && c != '.' {
                symbol_coords.insert(format!("{}{}", row, col));
            }
        }
    }

    // Find numbers and check for an adjacent symbol
    let mut sum = 0;
    for (row, line) in input.iter().enumerate() {
        let mut part_number = "".to_string();
        let mut adjacent_symbol = false;

        for (col, c) in line.chars().enumerate() {
            if c.is_numeric() {
                // Number found
                part_number.push(c);
                // Check adjacent coords if not found already
                adjacent_symbol =
                    adjacent_symbol || check_adjacent(row as i32, col as i32, &symbol_coords)
            } else if !part_number.is_empty() {
                if adjacent_symbol {
                    sum += part_number
                        .parse::<u32>()
                        .expect("Unable to parse part number");
                }
                part_number.clear();
                adjacent_symbol = false;
            }
        }

        // Handle a number at the end of a line
        if !part_number.is_empty() && adjacent_symbol {
            sum += part_number
                .parse::<u32>()
                .expect("Unable to parse part number");
        }
    }

    let answer_2 = "ANSWER_2".to_string();
    (format!("{}", sum), format!("{}", answer_2))
}

fn check_adjacent(row: i32, col: i32, symbol_coords: &HashSet<String>) -> bool {
    for r in row - 1..row + 2 {
        for c in col - 1..col + 2 {
            let coord = format!("{}{}", r, c);
            if symbol_coords.contains(&coord) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day03_1() {
        assert_eq!(
            day03(vec![
                "467..114..".to_string(),
                "...&......".to_string(),
                "..35..633.".to_string(),
                "......#...".to_string(),
                "617*......".to_string(),
                ".....+.58.".to_string(),
                "..592.....".to_string(),
                "......755.".to_string(),
                "...$.*....".to_string(),
                ".664.598..".to_string(),
            ])
            .0,
            "4361"
        )
    }

    #[test]
    fn day03_2() {
        assert_eq!(day03(vec!["".to_string()]).1, "ANSWER_2")
    }
}
