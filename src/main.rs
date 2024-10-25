use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod parser;

type DayFn = fn(Vec<String>) -> (String, String);
const DAYS: [DayFn; 7] = [
    day01::day01,
    day02::day02,
    day03::day03,
    day04::day04,
    day05::day05,
    day06::day06,
    day07::day07,
];

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please specify a single day as an integer")
    }
    let day = args[1]
        .parse::<usize>()
        .expect("Day parameter not an integer");

    let input = parser::load_input(day);
    let (part1, part2) = DAYS[day - 1](input);
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
