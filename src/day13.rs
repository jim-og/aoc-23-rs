fn get_patterns(input: &[String]) -> Vec<Vec<Vec<char>>> {
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
    patterns
}

fn get_candidates(pattern: &[Vec<char>]) -> Vec<usize> {
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

    // Scan each row, eliminating invalid candidates
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

    candidates
}

fn get_mirror(
    pattern: &[Vec<char>],
    prev: (Option<usize>, Option<usize>),
) -> Option<(Option<usize>, Option<usize>)> {
    // Search for vertical mirror line
    for candidate in get_candidates(pattern) {
        let mirror = (Some(candidate), None);
        if mirror != prev {
            return Some(mirror);
        }
    }

    // Search for horizontal mirror line
    for candidate in get_candidates(&transpose(pattern)) {
        let mirror = (None, Some(candidate));
        if mirror != prev {
            return Some(mirror);
        }
    }

    None
}

fn get_clean_mirror(pattern: &[Vec<char>]) -> Option<(Option<usize>, Option<usize>)> {
    let prev = get_mirror(pattern, (None, None)).expect("No mirror found");
    let width = pattern
        .first()
        .expect("Expected pattern to have a row")
        .len();
    let height = pattern.len();
    for i in 0..height {
        for j in 0..width {
            let mut clean_pattern = pattern.to_owned();
            clean_pattern[i][j] = match clean_pattern[i][j] {
                '.' => '#',
                '#' => '.',
                _ => unreachable!(),
            };
            match get_mirror(&clean_pattern, prev) {
                None => continue,
                Some(mirror) => return Some(mirror),
            }
        }
    }
    None
}

fn is_symmetrical(line: &[char], index: usize) -> bool {
    let min_width = index.min(line.len() - index);
    let left = &line[index - min_width..index];
    let right = &mut line.to_owned()[index..index + min_width];
    right.reverse();
    left == right
}

fn transpose<T>(v: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn part_1(patterns: &[Vec<Vec<char>>]) -> usize {
    let scores = patterns
        .iter()
        .map(
            |pattern| match get_mirror(pattern, (None, None)).expect("No mirror found") {
                (Some(s), None) => s,
                (None, Some(s)) => s * 100,
                _ => unimplemented!(),
            },
        )
        .collect::<Vec<_>>();

    scores.iter().sum::<usize>()
}

fn part_2(patterns: &[Vec<Vec<char>>]) -> usize {
    let scores = patterns
        .iter()
        .map(
            |pattern| match get_clean_mirror(pattern).expect("No mirror found") {
                (Some(s), None) => s,
                (None, Some(s)) => s * 100,
                _ => unimplemented!(),
            },
        )
        .collect::<Vec<_>>();

    scores.iter().sum::<usize>()
}

pub fn day13(input: Vec<String>) -> (String, String) {
    let patterns = get_patterns(&input);
    let part_1 = part_1(&patterns);
    let part_2 = part_2(&patterns);
    (format!("{}", part_1), format!("{}", part_2))
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
        "405",
        "400"
        ;"1"
    )]
    fn example(input: &str, part_1: &str, part_2: &str) {
        let data = parser::test_input(input);
        let result = day13(data);
        assert_eq!(result.0, part_1);
        assert_eq!(result.1, part_2);
    }

    #[test]
    fn mainline() {
        let input = parser::load_input(13);
        let result = day13(input);
        assert_eq!(result.0, "33975");
        assert_eq!(result.1, "29083");
    }
}
