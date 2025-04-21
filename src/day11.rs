use crate::parser;

pub fn solve(input: &[String], expansion_factor: usize) -> usize {
    let mut image = Vec::new();
    let mut empty_row = Vec::new();
    let mut empty_col = Vec::new();
    let width = input.first().expect("Input is empty").len();

    input.iter().enumerate().for_each(|(row, line)| {
        let mut empty_space = 0;
        line.chars().enumerate().for_each(|(col, c)| {
            if c == '#' {
                image.push((col, row));
            } else {
                empty_space += 1;
            }
        });
        if empty_space == width {
            empty_row.push(row);
        }
    });

    // Check for empty columns
    for col in 0..width {
        if !image.iter().any(|(c, _)| *c == col) {
            empty_col.push(col);
        }
    }

    // Taxicab distance (https://en.wikipedia.org/wiki/Taxicab_geometry)
    let mut distances = Vec::new();
    for i in 0..image.len() - 1 {
        for j in i + 1..image.len() {
            let p1 = image[i];
            let p2 = image[j];
            let (col_min, col_max) = (p1.0.min(p2.0), p1.0.max(p2.0));
            let (row_min, row_max) = (p1.1.min(p2.1), p1.1.max(p2.1));
            let col_diff = col_max - col_min;
            let row_diff = row_max - row_min;
            // Add row and col expansions
            let col_expansion = empty_col
                .iter()
                .filter(|c| **c >= col_min && **c <= col_max)
                .count();
            let row_expansion = empty_row
                .iter()
                .filter(|r| **r >= row_min && **r <= row_max)
                .count();
            let distance = col_diff
                + col_expansion * (expansion_factor - 1)
                + row_diff
                + row_expansion * (expansion_factor - 1);
            distances.push(distance);
        }
    }

    distances.iter().sum::<usize>()
}

#[aoc_generator(day11)]
pub fn parse(input: &str) -> Vec<String> {
    parser::test_input(input)
}

#[aoc(day11, part1)]
pub fn part1(input: &[String]) -> usize {
    solve(input, 2)
}

#[aoc(day11, part2)]
pub fn part2(input: &[String]) -> usize {
    solve(input, 1000000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use test_case::test_case;

    #[test_case(
        "
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
        ",
        374,
        1030,
        8410
        ;"e1"
    )]
    fn both_test(input: &str, answer_1: usize, answer_2: usize, answer_3: usize) {
        let data = parse(input);
        assert_eq!(solve(&data, 2), answer_1);
        assert_eq!(solve(&data, 10), answer_2);
        assert_eq!(solve(&data, 100), answer_3);
    }

    #[test]
    fn mainline() {
        let input = parse(&parser::load_input_string(11));
        assert_eq!(part1(&input), 9648398);
        assert_eq!(part2(&input), 618800410814);
    }
}
