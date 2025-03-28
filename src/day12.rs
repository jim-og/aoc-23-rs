use std::collections::HashMap;

use crate::parser;

struct Record {
    springs: String,
    groups: Vec<usize>,
}

fn parse_input(input: &Vec<String>, repeat: usize) -> Vec<Record> {
    let mut records = Vec::new();
    for line in input {
        let split = line.split(' ').collect::<Vec<&str>>();
        let springs = split
            .first()
            .expect("Expected condition records for springs")
            .split(',') // bit of a hack to get an iterator
            .collect::<Vec<&str>>()
            .repeat(repeat)
            .join("?");
        let groups = split
            .last()
            .expect("Expected spring arrangements")
            .split(',')
            .map(|x| x.parse::<usize>().expect("Failed to parse arrangement"))
            .collect::<Vec<usize>>()
            .repeat(repeat);
        records.push(Record { springs, groups });
    }
    records
}

/// Solving using top-down Dynamic Programming
fn solve(
    springs: &str,
    groups: &[usize],
    memo: &mut HashMap<(String, Vec<usize>), usize>,
) -> usize {
    // If there are no more arrangements left, we've reached the end.
    // There must be no broken springs ('#') remaining for this to be a solution.
    if groups.is_empty() {
        if springs.contains('#') {
            return 0;
        } else {
            return 1;
        }
    }

    // Check whether we've previously calculated this solution
    if let Some(res) = memo.get(&(springs.to_string(), groups.to_vec())) {
        return *res;
    }

    // Check whether the remaining groups can possibly be made from the remaining springs.
    // Remember that each group must have an operational spring between them.
    let length_required = groups.iter().sum::<usize>() + groups.len() - 1;
    if springs.len() < length_required {
        return 0;
    }

    // Skip this spring if it is operational. I find the str slice notation a little confusing
    // but this is effectively checking whether index 0 is operational. If it is, check the
    // remaining springs from index 1 onwards.
    let current_spring = &springs[0..1];
    if current_spring == "." {
        return solve(&springs[1..], groups, memo);
    }

    // Check whether the next damaged group can be placed at this position.
    // For this to hold true, it must not have any operational springs ('.').
    let group = *groups.first().expect("Groups unexpectedly empty");
    let all_broken = !springs[0..group].contains(".");

    // Check that this is either the last group, or that the spring after this group
    // is not explicitly broken as there must be at least one operational spring between broken groups.
    let placement_valid = springs.len() == group || !springs[group..group + 1].contains("#");

    // Consider this group is placed here. Find out whether the remaining groups fit into the
    // remaining springs. Remembering to skip the next spring if we're not at the end.
    let mut total = 0;
    if all_broken && placement_valid {
        let next_index = springs.len().min(group + 1);
        total += solve(&springs[next_index..], &groups[1..], memo);
    }

    // If this spring is '?' it could be operational, in which case we should skip it and
    // consider whether these groups fit into the remaining springs.
    if current_spring == "?" {
        total += solve(&springs[1..], groups, memo);
    }

    // Add this answer to the memo
    memo.insert((springs.to_string(), groups.to_vec()), total);
    total
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<String> {
    parser::test_input(input)
}

#[aoc(day12, part1)]
pub fn part1(input: &Vec<String>) -> usize {
    let records_1 = parse_input(input, 1);
    let mut part_1 = Vec::new();
    for record in records_1 {
        part_1.push(solve(&record.springs, &record.groups, &mut HashMap::new()));
    }
    part_1.iter().sum::<usize>()
}

#[aoc(day12, part2)]
pub fn part2(input: &Vec<String>) -> usize {
    let records_2 = parse_input(input, 5);
    let mut part_2 = Vec::new();
    for record in records_2 {
        part_2.push(solve(&record.springs, &record.groups, &mut HashMap::new()));
    }
    part_2.iter().sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn both_test() {
        let input = input_generator(
            "???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1",
        );
        assert_eq!(part1(&input), 21);
        assert_eq!(part2(&input), 525152);
    }

    #[test]
    fn mainline() {
        let input = input_generator(&parser::load_input_string(12));
        assert_eq!(part1(&input), 7307);
        assert_eq!(part2(&input), 3415570893842);
    }
}
