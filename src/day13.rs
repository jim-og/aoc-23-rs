trait Symmetrical {
    fn is_symmetrical(&self, index: usize) -> bool;
}

impl Symmetrical for Vec<char> {
    fn is_symmetrical(&self, index: usize) -> bool {
        let min_width = index.min(self.len() - index);
        let left = &self[index - min_width..index];
        let right = &mut self.to_owned()[index..index + min_width];
        right.reverse();
        left == right
    }
}

type Pattern = Vec<Vec<char>>;

trait Mirror {
    fn get_candidates(&self) -> Vec<usize>;
    fn get_mirror(
        &self,
        prev: (Option<usize>, Option<usize>),
    ) -> Option<(Option<usize>, Option<usize>)>;
    fn get_clean_mirror(&self) -> Option<(Option<usize>, Option<usize>)>;
    fn transpose(&self) -> Pattern;
}

impl Mirror for Pattern {
    fn get_candidates(&self) -> Vec<usize> {
        let mut candidates = Vec::new();
        let width = self.first().expect("Pattern empty").len();

        // Scan the first row for candidates
        if let Some(line) = self.first() {
            for i in 1..width {
                if line.is_symmetrical(i) {
                    candidates.push(i);
                }
            }
        }

        // Scan each row, eliminating invalid candidates
        for (index, line) in self.iter().enumerate() {
            if index == 0 {
                continue;
            }
            if candidates.is_empty() {
                break;
            }
            candidates = candidates
                .into_iter()
                .filter(|c| line.is_symmetrical(*c))
                .collect::<Vec<usize>>();
        }

        candidates
    }

    fn get_mirror(
        &self,
        prev: (Option<usize>, Option<usize>),
    ) -> Option<(Option<usize>, Option<usize>)> {
        // Search for vertical mirror line
        for candidate in self.get_candidates() {
            let mirror = (Some(candidate), None);
            if mirror != prev {
                return Some(mirror);
            }
        }

        // Search for horizontal mirror line
        for candidate in self.transpose().get_candidates() {
            let mirror = (None, Some(candidate));
            if mirror != prev {
                return Some(mirror);
            }
        }

        None
    }

    fn get_clean_mirror(&self) -> Option<(Option<usize>, Option<usize>)> {
        let prev = self.get_mirror((None, None)).expect("No mirror found");
        let width = self.first().expect("Expected pattern to have a row").len();
        let height = self.len();
        for i in 0..height {
            for j in 0..width {
                let mut clean_pattern = self.to_owned();
                clean_pattern[i][j] = match clean_pattern[i][j] {
                    '.' => '#',
                    '#' => '.',
                    _ => unreachable!(),
                };
                match clean_pattern.get_mirror(prev) {
                    None => continue,
                    Some(mirror) => return Some(mirror),
                }
            }
        }
        None
    }

    fn transpose(&self) -> Pattern {
        inner_transpose(self)
    }
}

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

fn inner_transpose<T>(v: &[Vec<T>]) -> Vec<Vec<T>>
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
            |pattern| match pattern.get_mirror((None, None)).expect("No mirror found") {
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
            |pattern| match pattern.get_clean_mirror().expect("No mirror found") {
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
