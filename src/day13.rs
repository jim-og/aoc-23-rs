pub fn solve(input: &[String]) -> usize {
    let mut patterns = Vec::new();
    let mut pattern = Vec::new();
    for line in input {
        if line.is_empty() {
            patterns.push(pattern.clone());
            pattern.clear();
            continue;
        }
        pattern.push(line.chars().collect::<Vec<_>>());
    }
    patterns.push(pattern);

    let mut part_1 = Vec::new();
    for pattern in patterns {
        if let Some(vertical_line) = get_mirror_index(&pattern) {
            part_1.push(vertical_line);
        } else if let Some(horizontal_line) = get_mirror_index(&transpose(&pattern)) {
            part_1.push(100 * horizontal_line);
        } else {
            panic!("No line of reflection found");
        }
    }

    part_1.iter().sum::<usize>()
}

fn get_mirror_index(pattern: &Vec<Vec<char>>) -> Option<usize> {
    let mut candidates = Vec::new();
    let width = pattern.first().expect("Pattern empty").len();

    // Scan the first row for candidates
    if let Some(line) = pattern.first() {
        for i in 1..width {
            if is_symmetrical(line, i) {
                candidates.push(i);
            }
        }
    }

    // Scan each row
    for (index, line) in pattern.iter().enumerate() {
        if index == 0 {
            continue;
        }
        if candidates.is_empty() {
            break;
        }
        candidates = candidates
            .into_iter()
            .filter(|c| is_symmetrical(line, *c))
            .collect::<Vec<usize>>();
    }

    if candidates.len() == 1 {
        return Some(*candidates.first().expect("Expected a candidate"));
    } else {
        None
    }
}

fn is_symmetrical(line: &Vec<char>, index: usize) -> bool {
    let min_width = index.min(line.len() - index);
    let left = &line[index - min_width..index];
    let right = &mut line.clone()[index..index + min_width];
    right.reverse();
    left == right
}

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

pub fn day13(input: Vec<String>) -> (String, String) {
    let part_1 = solve(&input);
    (format!("{}", part_1), format!("{}", 0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use test_case::test_case;

    #[test_case(
        "
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
        ",
        405
        ;"1"
    )]
    fn example(input: &str, answer: usize) {
        let data = parser::test_input(input);
        assert_eq!(solve(&data), answer);
    }

    #[test]
    fn mainline() {
        let input = parser::load_input(13);
        let result = day13(input);
        assert_eq!(result.0, "33975");
        assert_eq!(result.1, "0");
    }
}
