#[aoc(day15, part1)]
fn part1(input: &str) -> u32 {
    input
        .trim()
        .split(',')
        .fold(0, |acc, step| acc + hash(step))
}

fn hash(label: &str) -> u32 {
    label.chars().fold(0, |acc, c| (acc + c as u32) * 17 % 256)
}

#[aoc(day15, part2)]
fn part2(_input: &str) -> u32 {
    145
}

#[cfg(test)]
mod tests {
    use crate::parser;

    use super::*;
    use test_case::test_case;

    #[test_case(
        "HASH", 
        52
        ;"e1"
    )]
    #[test_case(
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", 
        1320
        ;"e2"
    )]
    #[test_case(
        "qp", 
        1
        ;"e3"
    )]
    fn part1_example(input: &str, want: u32) {
        assert_eq!(part1(input), want);
    }

    #[test_case(
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", 
        145
        ;"e1"
    )]
    fn part2_example(input: &str, want: u32) {
        assert_eq!(part2(input), want);
    }

    #[test]
    fn mainline() {
        let input = &parser::load_input_string(15);
        assert_eq!(part1(input), 522547);
    }
}
