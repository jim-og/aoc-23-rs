use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, PartialEq, Clone)]
enum Outcome {
    Accept,
    Reject,
    Send,
}

#[derive(Debug, PartialEq, Clone)]
enum Comparator {
    GreaterThan,
    LessThan,
}

#[derive(Debug, PartialEq, Clone)]
struct PartRating {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl PartRating {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Rule {
    category: Option<Category>,
    comparator: Option<Comparator>,
    value: Option<usize>,
    outcome: Outcome,
    send_to: Option<String>,
}

struct Input {
    workflows: HashMap<String, Vec<Rule>>,
    parts: Vec<PartRating>,
}

fn parse_part_rating(line: &str) -> PartRating {
    let trimmed_line = line
        .trim_matches(['{', '}'])
        .split(',')
        .collect::<Vec<&str>>();

    let ratings: HashMap<Category, usize> = trimmed_line
        .iter()
        .map(|rating| {
            let equation = rating.split('=').collect::<Vec<&str>>();
            let category = match *equation
                .first()
                .expect("Failed to parse part rating category")
            {
                "x" => Category::X,
                "m" => Category::M,
                "a" => Category::A,
                "s" => Category::S,
                _ => panic!("Invalid category"),
            };
            let value = equation
                .get(1)
                .expect("Failed to get part rating value")
                .parse::<usize>()
                .expect("Failed to parse part rating value");
            (category, value)
        })
        .collect();

    PartRating {
        x: *ratings.get(&Category::X).unwrap(),
        m: *ratings.get(&Category::M).unwrap(),
        a: *ratings.get(&Category::A).unwrap(),
        s: *ratings.get(&Category::S).unwrap(),
    }
}

fn parse_workflow(line: &str) -> (String, Vec<Rule>) {
    // TODO fixup verbose input parsing
    let mut rules = Vec::new();
    let n = line.split('{').collect::<Vec<&str>>();
    let name = n.first().expect("Expected workflow name");
    let workflow = n
        .get(1)
        .expect("Expected workflow instructions")
        .trim_matches('}');
    for instruct in workflow.split(',') {
        if !instruct.contains('<') && !instruct.contains('>') {
            let (outcome, send_to) = match instruct {
                "A" => (Outcome::Accept, None),
                "R" => (Outcome::Reject, None),
                _ => (Outcome::Send, Some(instruct.to_string())),
            };
            rules.push(Rule {
                category: None,
                comparator: None,
                value: None,
                outcome,
                send_to,
            });
        } else {
            let mut chars = instruct.chars();
            let category = match chars
                .next()
                .expect("Expected instruction to start with part category")
            {
                'x' => Some(Category::X),
                'm' => Some(Category::M),
                'a' => Some(Category::A),
                's' => Some(Category::S),
                _ => panic!("Invalid category"),
            };
            let comparator = match chars
                .next()
                .expect("Expected instruction to contain a comparator")
            {
                '<' => Some(Comparator::LessThan),
                '>' => Some(Comparator::GreaterThan),
                _ => panic!("Invalid comparator"),
            };
            let segment = &instruct[2..].split(':').collect::<Vec<&str>>();
            let value = Some(
                segment
                    .first()
                    .expect("Expected instruction to have a value")
                    .parse::<usize>()
                    .expect("Unable to parse value"),
            );
            let outcome_str = segment
                .get(1)
                .expect("Expected instruction to have a outcome");
            let (outcome, send_to) = match *outcome_str {
                "A" => (Outcome::Accept, None),
                "R" => (Outcome::Reject, None),
                _ => (Outcome::Send, Some(outcome_str.to_string())),
            };
            rules.push(Rule {
                category,
                comparator,
                value,
                outcome,
                send_to,
            });
        }
    }

    (name.to_string(), rules)
}

trait Workflow {
    fn apply_workflow(&self, part: &PartRating) -> (Outcome, Option<String>);
}

impl Workflow for Vec<Rule> {
    fn apply_workflow(&self, part: &PartRating) -> (Outcome, Option<String>) {
        for rule in self {
            match &rule.category {
                Some(category) => {
                    let value = match category {
                        Category::X => part.x,
                        Category::M => part.m,
                        Category::A => part.a,
                        Category::S => part.s,
                    };
                    let rule_value = rule.value.expect("Expected rule to have value");
                    let apply_rule = match rule
                        .comparator
                        .clone()
                        .expect("Expected rule to have comparator")
                    {
                        Comparator::GreaterThan => value > rule_value,
                        Comparator::LessThan => value < rule_value,
                    };
                    if apply_rule {
                        return (rule.outcome.clone(), rule.send_to.clone());
                    }
                }

                None => return (rule.outcome.clone(), rule.send_to.clone()),
            }
        }

        panic!("No Outcome from workflow")
    }
}

#[aoc_generator(day19)]
fn parse(input: &str) -> Input {
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();

    for entry in input.trim().lines() {
        let line = entry.trim();
        if line.starts_with("{") {
            // It's a part rating
            parts.push(parse_part_rating(line));
        } else if line.is_empty() {
            // It's the empty line between workflows and part ratings
            continue;
        } else {
            // It's a workflow
            let workflow = parse_workflow(line);
            workflows.insert(workflow.0, workflow.1);
        }
    }

    Input { workflows, parts }
}

#[aoc(day19, part1)]
fn part1(input: &Input) -> usize {
    let parts = input.parts.clone();
    let workflows = input.workflows.clone();
    let mut result = 0;

    for part in parts {
        let mut outcome = Outcome::Send;
        let mut send_to = Some("in".to_string());
        while outcome == Outcome::Send {
            let workflow = workflows
                .get(&send_to.clone().expect("Expected send_to to be Some"))
                .expect("Expected workflow to exist");
            (outcome, send_to) = workflow.apply_workflow(&part);
        }
        if outcome == Outcome::Accept {
            result += part.sum();
        }
    }

    result
}

#[aoc(day19, part2)]
fn part2(_input: &Input) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use test_case::test_case;

    const TEST: &str = "
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
    ";

    #[test_case("{x=787,m=2655,a=1222,s=2876}", PartRating{
        x: 787,
        m: 2655,
        a: 1222,
        s: 2876,
        }; "a")]
    fn part_ratings(input: &str, answer: PartRating) {
        assert_eq!(parse_part_rating(input), answer);
    }

    #[test_case("px{a<2006:qkq,m>2090:A,rfg}", (
        "px".to_string(), 
        Vec::from(vec![
            Rule{
                category: Some(Category::A),
                comparator: Some(Comparator::LessThan),
                value: Some(2006),
                outcome: Outcome::Send,
                send_to: Some("qkq".to_string())
            },
            Rule{
                category: Some(Category::M),
                comparator: Some(Comparator::GreaterThan),
                value: Some(2090),
                outcome: Outcome::Accept,
                send_to: None
            },
            Rule{
                category: None,
                comparator: None,
                value: None,
                outcome: Outcome::Send,
                send_to: Some("rfg".to_string())
            }
        ]))
        ; "a")]
    fn workflow(input: &str, answer: (String, Vec<Rule>)) {
        assert_eq!(parse_workflow(input), answer);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST)), 19114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn mainline() {
        assert_eq!(part1(&parse(&parser::load_input_string(19))), 382440);
    }
}
